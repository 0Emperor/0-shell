use std::fs;

pub fn cp(args: Vec<String>) {
    if args.len() != 2 {
        eprintln!("cp: missing source or destination");
        return;
    }
    let src = &args[0];
    let dst = &args[1];
    match fs::metadata(&src) {
        Ok(meta) => {
            if meta.is_dir() {
                eprintln!("cp: '{}' is a directory (would ve used -r but not implemented)", src);
                return;
            }
        }
        Err(e) => {
            eprintln!("cp: cannot access '{}': {}", src, e);
            return;
        }
    }

    if let Err(e) = fs::copy(&src, dst) {
        eprintln!("cp: error copying '{}': {}", src, e);
    }
}
