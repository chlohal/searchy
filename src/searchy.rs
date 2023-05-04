mod path_executables;

use path_executables::path_executable_search::path_executables;

fn main() {
    for f in path_executables() {
        println!("{}", f.to_string_lossy());
    }
    
}