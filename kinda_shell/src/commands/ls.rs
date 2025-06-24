use std::fs;
pub fn ls(args: Vec<String>) {
    let mut l = false;
    let mut F = false;
    let mut a = false;
    for pflag in &args {
        if pflag.starts_with("-") {
            if pflag.contains("F") {
                F = true;
            }
            if pflag.contains("l") {
                l = true;
            }
            if pflag.contains("a") {
                a = true;
            }
        }
    }
    let mut h = false;
    for arg in args {
        if arg.starts_with("-") {
            continue;
        }
        h = true;
        browse(arg, a, F, l);
    }
    if !h {
        browse(
            std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            a,
            F,
            l,
        );
    }
}
fn browse(path: String, a: bool, F: bool, l: bool) {
    match fs::metadata(&path) {
        Err(e) => eprint!("ls: {path}: {e}"),
        Ok(mdata) => {
            if mdata.is_dir() {
                if let Ok(parent) = fs::read_dir(&path) {
                    for childs in parent {
                        if let Ok(c_name) = childs {
                            if let Ok(c_data) = c_name.metadata() {
                                println!("{:?} {:?} {:?}",c_data.permissions().readonly(),c_data.modified().unwrap(),c_name.path())
                            }
                        }
                    }
                } else {
                    eprintln!("ls: {path}: error reading dir")
                };
            } else {
                println!("{path}")
            }
        }
    };
}
