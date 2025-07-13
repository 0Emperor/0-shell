/// Clears the terminal screen by printing ANSI escape codes.
///
/// # Arguments
///
/// * `a` - Vector of options; this command does not accept any options.
///
/// # Behavior
///
/// - Prints an error if any options are passed.
/// - Clears the screen and moves the cursor to the top-left corner.
///
/// # Example
///
/// ```
/// clear(vec![]);
/// ```
pub fn clear(a:Vec<String>){
if a.len() != 0{
    eprintln!("clear doesnt work with options");
    return;
}
 print!("\x1B[2J\x1B[3J\x1B[H")
}
