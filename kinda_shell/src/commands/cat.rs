use std::fs;
pub fn cat(args:Vec<String>){
    for file in args {
            match fs::read_to_string(&file){
                Ok(contents)=>println!("{}",contents),
                Err(e) =>eprintln!("cat: {file}: {e}"),
            };
    }
}