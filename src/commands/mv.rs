use std::fs;
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
        println!("missing file operand or destination");
        return;
    }
    // let dst = &args[args.len() - 1];
    // let dst_path = Path::new(dst);
    let dst = match fs::canonicalize(&args[args.len() - 1]) {
        Ok(d) => d,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // println!("abs path:{} -- {}", dst.is, dst_path.display());

    if args.len() > 2 {
        if !dst.is_dir() {
            println!("target {} is not a directory", dst.display());
            return;
        }
    } else if !dst.exists() {
        match fs::rename(args[0].clone(), dst.clone()) {
            Ok(_) => println!("I am worked, {} {}", args[0], dst.display()),
            Err(e) => eprintln!("Error: {}", e),
        }
        return;
    }

    for arg in args[0..args.len() - 1].iter() {
        let file = match fs::canonicalize(arg) {
            Ok(d) => d,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
        match fs::rename(
            file.clone(),
            format!("{}/{}", dst.display(), file.display()),
        ) {
            Ok(_) => println!("I am worked, {arg} {}", dst.display()),
            Err(e) => {
                println!("{} | {}", dst.display(), file.display());
                eprintln!("Error: {}", e)
            }
        }
    }
}
