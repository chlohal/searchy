use std::{ffi::OsString, process::{Command, Stdio}};

pub fn run_shell_command(command_name: impl Into<OsString>, open_terminal: bool) -> Result<String, String> {

    let mut bash_command = Into::<OsString>::into(command_name);
    bash_command.extend(vec![OsString::from("; exec $SHELL")]);

    let mut cmd: Command = Command::new("nohup");

    if open_terminal {
        cmd
        .arg("x-terminal-emulator")
        .arg("-e");
    }

    cmd
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
    let _pid = child.id();

    Ok("".into())
}