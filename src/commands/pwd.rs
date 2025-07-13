/// Prints the current working directory.
///
/// # Arguments
///
/// * `cdir` - Reference to a string representing the current directory path.
///
/// # Example
///
/// ```
/// pwd(&std::env::current_dir().unwrap().display().to_string());
/// ```
pub fn pwd(cdir :&String) {
        println!("{}", cdir);
}