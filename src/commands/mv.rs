use std::{fs, path::Path};
// use std::path::Path;
/// Moves or renames files and directories.
///
/// # Arguments
///
/// * `args` - Vector of command-line arguments where:
///   - All but last are source paths.
///   - Last is the destination path.
///
/// # Behavior
///
/// - If multiple sources, destination must be an existing directory.
/// - If single source and destination does not exist, renames source to destination.
/// - Prints error messages for invalid arguments or I/O errors.
///
/// # Example
///
/// ```
/// mv(vec!["file1.txt".to_string(), "file2.txt".to_string(), "dir".to_string()]);
/// mv(vec!["oldname.txt".to_string(), "newname.txt".to_string()]);
/// ```
pub fn mv(args: Vec<String>) {
    if args.len() < 2 {
        eprintln!("mv: missing file operand or destination");
        return;
    }

    let source_paths: Vec<&Path> = args[0..args.len() - 1].iter().map(Path::new).collect();
    let dest_path = Path::new(&args[args.len() - 1]);

    let handle_rename = |res: Result<(), std::io::Error>, src_disp: &str, dest_disp: &str| {
        if let Err(e) = res {
            eprintln!("mv: cannot move '{}' to '{}': {}", src_disp, dest_disp, e);
            return false;
        }
        true
    };

    if source_paths.len() > 1 {
        if !dest_path.is_dir() {
            eprintln!("mv: target '{}' is not a directory", dest_path.display());
            return;
        }

        for src_path in source_paths {
            let file_name = match src_path.file_name() {
                Some(name) => name,
                None => {
                    eprintln!(
                        "mv: cannot stat '{}': No such file or directory",
                        src_path.display()
                    );
                    return;
                }
            };
            let final_dest = dest_path.join(file_name);

            if !handle_rename(
                fs::rename(src_path, &final_dest),
                &src_path.to_string_lossy(),
                &final_dest.to_string_lossy(),
            ) {
                return;
            }
        }
    } else {
        let src_path = source_paths[0];

        if dest_path.is_dir() {
            let file_name = match src_path.file_name() {
                Some(name) => name,
                None => {
                    eprintln!(
                        "mv: cannot stat '{}': No such file or directory",
                        src_path.display()
                    );
                    return;
                }
            };
            let final_dest = dest_path.join(file_name);
            handle_rename(
                fs::rename(src_path, &final_dest),
                &src_path.to_string_lossy(),
                &final_dest.to_string_lossy(),
            );
        } else {
            handle_rename(
                fs::rename(src_path, dest_path),
                &src_path.to_string_lossy(),
                &dest_path.to_string_lossy(),
            );
        }
    }
}
