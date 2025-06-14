use std::fs;
pub fn cat(args:Vec<String>){
    for file in args {
            match fs::read_to_string(&file){
                Ok(contents)=>println!("{}",contents),
                Err(e) =>eprintln!("cat: {file}: {e}"),
            };
    }
}
// when we enter only cat command we should enable the user to write in the stdin