use std::fs;
use std::path::Path;
/// Removes files or directories specified in the arguments.
///
/// Supports the `-r` flag for r_dir removal of directories.
///
/// # Arguments
///
/// * `args` - Vector of strings representing command-line arguments,
///            e.g. `["-r", "folder", "file.txt"]`
///
/// # Behavior
///
/// - If an unknown flag is found (anything other than `-r`), prints an error and exits.
/// - If no paths are provided, prints a missing operand error.
/// - Deletes files directly.
/// - Deletes directories only if `-r` flag is present; otherwise, prints an error.
///
/// # Example
///
/// ```
/// rm(vec!["-r".to_string(), "mydir".to_string()]);
/// rm(vec!["file.txt".to_string()]);
/// ```
pub fn rm(args: Vec<String>) {
    let mut r_dir = false;
    let mut paths = Vec::new();

    for arg in &args {
        let arg = arg.as_str();
        if arg == "-r"  {
            r_dir = true;
        } else if !arg.starts_with('-') {
            paths.push(arg);
        } else {
            eprintln!("rm: invalid option -- '{}'", arg.chars().nth(1).unwrap_or(' '));
            return;
        }
    }
    
    if paths.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }

    for path_str in paths {
        let path = Path::new(path_str);
        if !path.exists() {
            eprintln!("rm: cannot remove '{}': No such file or directory", path_str);
            continue;
        }

        if path.is_dir() {
            if r_dir {
                if let Err(e) = fs::remove_dir_all(path) {
                    eprintln!("rm: cannot remove '{}': {}", path_str, e);
                }
            } else {
                eprintln!("rm: cannot remove '{}': Is a directory", path_str);
            }
        } else {
            if let Err(e) = fs::remove_file(path) {
                eprintln!("rm: cannot remove '{}': {}", path_str, e);
            }
        }
    }
}
