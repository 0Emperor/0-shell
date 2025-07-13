use std::fs;
use std::io;
/// Prints the contents of files to standard output, or reads from stdin if no files provided.
///
/// # Arguments
///
/// * `args` - Vector of filenames. If empty, reads from stdin.
///
/// # Behavior
///
/// - Reads and prints entire file content for each file argument.
/// - If no files are given, reads lines from stdin and prints them until EOF.
///
/// # Example
///
/// ```
/// cat(vec!["file.txt".to_string()]);
/// cat(vec![]); // reads from stdin
/// ```
pub fn cat(args:Vec<String>){
    for file in &args {
            match fs::read_to_string(&file){
                Ok(contents)=>println!("{}",contents),
                Err(e) =>eprintln!("cat: {file}: {e}"),
            };
    }
    if  args.len()==0{
         loop {
        let mut line = String::new();
        let bytes = io::stdin().read_line(&mut line).unwrap();
        if bytes == 0 {
            break;
        }
        print!("{line}");
    }
    }
}
// when we enter only cat command we should enable the user to write in the stdi ------- done