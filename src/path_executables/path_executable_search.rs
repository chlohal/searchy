use std::{
    collections::VecDeque,
    env,
    fs::{self, Metadata},
    os::unix::prelude::PermissionsExt,
    path::PathBuf,
};

pub fn path_executables() -> PathExecutableSearch {
    let path_dirs = env::var("PATH").unwrap_or("".to_string());

    PathExecutableSearch {
        queue: path_dirs
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

pub struct PathExecutableSearch {
    queue: VecDeque<PathBuf>,
}

impl Iterator for PathExecutableSearch {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.queue.is_empty() {
            let next_path = self.queue.pop_front()?;

            if let Ok(f_type) = fs::metadata(&next_path) {
                //If it's an executable file, yield it
                if f_type.is_file() && is_executable(&f_type) {
                    return Some(next_path);
                }
                // Otherwise, if it's a directory, add all its children to the queue
                else if f_type.is_dir() {
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

fn is_executable(meta: &Metadata) -> bool {
    meta.permissions().mode() & 0o111 != 0
}
