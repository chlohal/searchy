use std::{path::PathBuf, ffi::OsString, process::{Command, Stdio}};

pub fn run_shell_command(command_name: &PathBuf) -> Result<String, String> {

    let mut bash_command = command_name.as_os_str().to_owned();
    bash_command.extend(vec![OsString::from("; $SHELL")]);

    let mut cmd: Command = Command::new("nohup");
    cmd
        .arg("x-terminal-emulator")
        .arg("-e")
        .arg("bash")
        .arg("-c")
        .arg(bash_command);

    let child = cmd.stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
        .map_err(|e| e.to_string())?;

    //get child's ID, which has the side effect of waiting for it to be fully
    //loaded by the system.
    let pid = child.id();

    Ok("".into())
}