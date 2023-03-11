use std::{collections::VecDeque, env, fs, path::PathBuf, process::{Command, Stdio}};

use application_file::ApplicationFile;
use searching::search_em_up;

mod application_file;
mod ini_file;
mod searching;

fn main() -> Result<(), ()> {

    match search_em_up(application_files()) {
        Some(app) => run_application(app).map(|_| ()),//{println!("{:?}", app); Ok(())},
        None => Err(()),
    }
}

fn run_application(app: ApplicationFile) -> Result<u32, ()> {
    let command_name = app.app_exec.ok_or(())?
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
        .replace("%i", &icon_arg(app.app_icon)) //replace %i with --icon <icon> if icon is specified
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
        .map_err(|_| ())?;

    //get child's ID, which has the side effect of waiting for it to be fully
    //loaded by the system.
    let pid = child.id();

    Ok(pid)
}

fn icon_arg(icon: Option<String>) -> String {
    match icon {
        Some(icon) => "--icon ".to_string() + &icon,
        None => "".to_string()
    }
}

fn application_files() -> ApplicationFileSearch {
    let data_dirs = env::var("XDG_DATA_DIRS").unwrap_or("".to_string());

    ApplicationFileSearch {
        queue: data_dirs
            .split(":")
            .filter_map(|path| {
                if path == "" {
                    None
                } else {
                    Some(PathBuf::from(path))
                }
            })
            .collect(),
    }
}

pub struct ApplicationFileSearch {
    queue: VecDeque<PathBuf>,
}

impl Iterator for ApplicationFileSearch {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.queue.is_empty() {
            let next_path = self.queue.pop_front()?;

            //If it ends with ".desktop", return it!
            if next_path.to_string_lossy().ends_with(".desktop") {
                return Some(next_path);
            }

            // If it's a directory, add all its children to the queue
            if let Ok(f_type) = fs::metadata(&next_path) {
                if f_type.is_dir() {
                    if let Ok(dirs) = fs::read_dir(next_path) {
                        for dir in dirs {
                            if let Ok(dir_entry) = dir {
                                self.queue.push_back(dir_entry.path());
                            }
                        }
                    }
                }
            }
        }
        None
    }
}
