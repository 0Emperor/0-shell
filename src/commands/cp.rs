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
                eprintln!(
                    "cp: '{}' is a directory (would ve used -r but not implemented)",
                    src
                );
                return;
            }
        }
        Err(e) => {
            eprintln!("cp: cannot access '{}': {}", src, e);
            return;
        }
    }

    match (
        fs::metadata(&dst),
        fs::canonicalize(dst),
        fs::canonicalize(src),
        std::env::current_dir(),
    ) {
        (Ok(meta), Ok(abs_dst_path), Ok(abs_src_path), Ok(cdr)) => {
            if abs_dst_path == cdr || abs_dst_path == abs_src_path {
                println!("You have been try to copy the same file!");
                return;
            }

            if meta.is_dir() {
                if let Err(e) = std::env::set_current_dir(dst) {
                    eprintln!("{}", e);
                    return;
                }

                if let Err(e) = fs::File::create(src) {
                    eprintln!("{}", e);
                    return;
                }
                if let Err(e) = std::env::set_current_dir("..") {
                    eprintln!("{}", e);
                    return;
                }
                if let Err(e) = fs::copy(&src, format!("{}/{}", dst, src)) {
                    eprintln!("{}", e);
                    return;
                }

                return;
            } else {
                if let Err(e) = fs::copy(&src, dst) {
                    eprintln!("cp: error copying '{}': {}", src, e);
                }
            }
        }
        (Err(e), _, _, _) => {
            eprintln!("cp: cannot access '{}': {}", src, e);
            return;
        }
        _ => {
            eprintln!("cp: Error");
            return;
        }
    }
}
