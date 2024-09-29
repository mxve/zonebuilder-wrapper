#![windows_subsystem = "windows"]
use native_dialog::*;
use std::env;

fn error(title: &str, text: &str) {
    eprintln!("---------{}---------\n---------{}---------", title, text);
    MessageDialog::new()
        .set_type(MessageType::Error)
        .set_title(title)
        .set_text(text)
        .show_alert()
        .unwrap();
}

fn main() {
    let iw4x = env::current_dir().unwrap().join("iw4x.exe");

    // make sure iw4x.exe exists
    if !iw4x.exists() {
        error("iw4x.exe not found!", "Could not find iw4x.exe, make sure zonebuilder.exe is placed in the root of MW2 with IW4x installed.");
        return;
    }

    // first arg is the path of this executable
    let args_current: Vec<String> = env::args().skip(1).collect();

    // if -stdout is set, we have to attach a console to display output
    if args_current.contains(&"-stdout".to_owned()) {
        unsafe {
            if winapi::um::wincon::AttachConsole(winapi::um::wincon::ATTACH_PARENT_PROCESS) == 0 {
                error("Failed to attach console", "-stdout is set but zonebuilder.exe failed to attach a console window.\nIf this issue persists please run iw4x.exe with the -zonebuilder flag directly.")
            }
        }
    }

    // prepend args with -zonebuilder
    let args_zonebuilder = [vec!["-zonebuilder".to_string()], args_current].concat();

    // build command
    let mut cmd = std::process::Command::new(iw4x);
    cmd.args(args_zonebuilder.clone());

    // run iw4x.exe with args_zonebuilder
    let exit_status = match cmd.spawn() {
        // if process spawned successfully, wait for it to exit
        Ok(mut child) => match child.wait() {
            // return exit code
            Ok(status) => status,
            Err(e) => {
                error("Failed to wait for iw4x.exe process!", &format!("An error occurred while waiting for the iw4x.exe process to complete: {:?}", e));
                return;
            }
        },
        Err(e) => {
            error(
                "Failed to run iw4x.exe!",
                &format!("Failed to execute iw4x.exe: {:?}", e),
            );
            return;
        }
    };

    // if we got an exit code, pass it through, otherwise default to 1
    if exit_status.code().is_some() {
        std::process::exit(exit_status.code().unwrap());
    }
    std::process::exit(1);
}
