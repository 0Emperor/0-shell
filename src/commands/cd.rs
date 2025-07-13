use std::env;
/// Changes the current working directory.
///
/// # Arguments
///
/// * `args` - Vector of arguments:
///   - If empty, changes to directory specified by $HOME.
///   - If contains exactly one element, changes to that directory.
///   - Otherwise, prints an error.
///
/// # Behavior
///
/// - Prints errors if changing directory fails or usage is incorrect.
///
/// # Example
///
/// ```
/// cd(vec![]); // changes to $HOME
/// cd(vec!["/tmp".to_string()]); // changes to /tmp
/// ```
pub fn cd(args: Vec<String>) {
    if args.is_empty() {
        if let Some(h) = env::var_os("HOME") {
            if let Err(e) = env::set_current_dir(h) {
                eprintln!("cd: {}",  e);
            }
        } else {
            eprintln!("cd: Unable to determine home directory");
        }
        return;
    }
    if args.len() != 1 {
        eprintln!("cd: expected exactly one argument");
        return;
    }

    let path = &args[0];
    if let Err(e) = env::set_current_dir(path) {
        eprintln!("cd: {}: {}", path, e);
    }
}
