use std::fs;
use std::path::Path;

pub fn mv(args: Vec<String>) {
    if args.len() < 2 {
        println!("missing file operand or destination");
        return;
    }
    let dst = &args[args.len() - 1];
    let dst_path = Path::new(dst);

    if args.len() > 2 {
        if !dst_path.is_dir() {
            println!("target {dst} is not a directory");
            return;
        }
    } else if !dst_path.exists() {
        match fs::rename(args[0].clone(), dst) {
            Ok(_) => println!("I am worked, {} {dst}", args[0]),
            Err(e) => eprintln!("Error: {}", e),
        }
        return;
    }

    for arg in args[0..args.len() - 1].iter() {
        match fs::rename(arg, format!("{dst}/{arg}")) {
            Ok(_) => println!("I am worked, {arg} {dst}"),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
// file1 file2
// file1 dir/
// file1 file2 file3 dir/
