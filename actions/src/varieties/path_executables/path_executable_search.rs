use std::{
    collections::{HashSet, VecDeque},
    env,
    fs::{self, Metadata},
    os::unix::prelude::PermissionsExt,
    path::PathBuf,
};

pub fn path_executables() -> PathExecutableSearch {
    let path_dirs = env::var("PATH").unwrap_or("".to_string());

    PathExecutableSearch {
        queue: path_dirs
            .split(':')
            .filter_map(|directory| {
                fs::read_dir(resolve_symlink(PathBuf::from(directory)))
                    .ok()
                    .map(|files| files.filter_map(|file| file.ok().map(|f| f.path())))
            })
            .flatten()
            .collect(),
        previously_generated: HashSet::new(),
    }
}

pub struct PathExecutableSearch {
    queue: VecDeque<PathBuf>,
    previously_generated: HashSet<PathBuf>,
}

fn resolve_symlink(path: PathBuf) -> PathBuf {
    if let Ok(meta) = fs::symlink_metadata(&path) {
        if meta.is_symlink() {
            fs::read_link(&path).unwrap_or(path)
        } else {
            path
        }
    } else {
        path
    }
}

impl Iterator for PathExecutableSearch {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.queue.is_empty() {
            let next_path = resolve_symlink(self.queue.pop_front()?);

            if let Ok(f_type) = fs::symlink_metadata(&next_path) {
                //If it's an executable file, yield it

                if f_type.is_symlink() {
                    if let Ok(pointed_path) = fs::read_link(next_path) {
                        self.queue.push_back(pointed_path);
                    }
                } else if is_executable(&f_type) && !self.previously_generated.contains(&next_path) {
                    self.previously_generated.insert(next_path.clone());
                    return Some(next_path);
                }
            }
        }
        None
    }
}

fn is_executable(meta: &Metadata) -> bool {
    meta.permissions().mode() & 0o111 != 0
}
