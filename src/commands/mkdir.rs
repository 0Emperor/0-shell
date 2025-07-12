use std::fs;
pub fn mkdir(args: Vec<String>) {
    for dir in args {
        if let Err(e) = fs::create_dir(&dir) {
            eprintln!("mkdir: {dir}: {e}, {}", e.kind())
        }
    }
}
