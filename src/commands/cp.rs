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

    match fs::metadata(&dst) {
        Ok(meta) => {
            if meta.is_dir() {
                if let Err(e) = std::env::set_current_dir(dst) {
                    eprintln!("{}", e);
                    return;
                }

                if let Err(e) = fs::File::create(src) {
                    eprintln!("{}", e);
                    return;
                }
                if dst != "." {
                    if let Err(e) = std::env::set_current_dir("..") {
                        eprintln!("{}", e);
                        return;
                    }
                    if let Err(e) = fs::copy(&src, format!("{}/{}", dst, src)) {
                        eprintln!("{}", e);
                        return;
                    }
                }

                return;
            } else {
                if let Err(e) = fs::copy(&src, dst) {
                    eprintln!("cp: error copying '{}': {}", src, e);
                }
            }
        }
        Err(e) => {
            eprintln!("cp: cannot access '{}': {}", src, e);
            return;
        }
    }
}
