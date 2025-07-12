use std::fs;
use std::path::Path;

pub fn rm(args: Vec<String>) {
    let mut recursive = false;
    let mut paths = Vec::new();

    for arg in &args {
        let arg = arg.as_str();
        if arg == "-r" {
            recursive = true;
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
            if recursive {
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
