use std::env;

pub fn cd(args: Vec<String>){
    if args.len()==0{
        if let Some(home_dir) = env::home_dir() {
            if let Err(e) = env::set_current_dir(&home_dir) {
                eprintln!("cd: {}: {}", home_dir.display(), e);
            }
        } else {
            eprintln!("cd: Unable to determine home directory");
        }

    return
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