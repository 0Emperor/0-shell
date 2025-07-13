use std::fs;
/// Creates directories specified in the argument vector.
///
/// # Arguments
///
/// * `args` - Vector of strings representing directory names to create.
///
/// # Behavior
///
/// - Prints an error if no arguments are given.
/// - Attempts to create each directory individually.
/// - Prints an error message if creation of any directory fails.
///
/// # Example
///
/// ```
/// mkdir(vec!["dir1".to_string(), "dir2".to_string()]);
/// ```
pub fn mkdir(args:Vec<String>){
    if args.len() ==0 {
                 eprintln!("mkdir: missing argupments")
    }
    for dir in args {
        if let Err(e)= fs::create_dir(&dir){
             eprintln!("mkdir: {dir}: {e}")
        }
    }
}