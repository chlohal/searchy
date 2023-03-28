use std::{fs, env, path::PathBuf, collections::VecDeque};

use super::application_file::{parse_application_file, ApplicationFile};

pub fn application_files() -> ApplicationFileSearch {
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
    type Item = ApplicationFile;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.queue.is_empty() {
            let next_path = self.queue.pop_front()?;

            //If it ends with ".desktop", return it!
            if next_path.to_string_lossy().ends_with(".desktop") {
                match parse_application_file(next_path) {
                    Ok(desktop_file) => return Some(desktop_file),
                    Err(_) => continue,
                }
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
