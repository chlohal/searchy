use std::process::{Stdio, Command};

use super::application_file::ApplicationFile;

pub fn run_application(app: &ApplicationFile) -> Result<u32, String> {
    let command_name = app.app_exec.as_ref().ok_or("No 'exec' property")?
        .to_owned()
        .replace("%u", "") //We don't open a file or a URL, so remove all the file/url specifiers
        .replace("%f", "")
        .replace("%F", "")
        .replace("%U", "")
        .replace("%d", "") //remove the deprecated specifiers
        .replace("%D", "")
        .replace("%n", "")
        .replace("%N", "") 
        .replace("%v", "") 
        .replace("%m", "") 
        .replace("%i", &icon_arg(&app.app_icon)) //replace %i with --icon <icon> if icon is specified
        .replace("%c", &app.app_name) //replace name
        .replace("%k", &app.file_address.to_string_lossy()); //replace file address


    let terminal = app.app_terminal.unwrap_or_default();

    let mut cmd = Command::new("nohup");

    if terminal {
        cmd
        .arg("x-terminal-emulator")
        .arg("-e")
        .arg("bash")
        .arg("-c")
        .arg(command_name + "; $SHELL")
    } else {
        cmd
        .arg("bash")
        .arg("-c")
        .arg(command_name)
    };

    let child = cmd.stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
        .map_err(|e| e.to_string())?;

    //get child's ID, which has the side effect of waiting for it to be fully
    //loaded by the system.
    let pid = child.id();

    Ok(pid)
}

fn icon_arg(icon: &Option<String>) -> String {
    match icon {
        Some(icon) => "--icon ".to_string() + icon,
        None => "".to_string()
    }
}