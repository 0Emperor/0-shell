use std::fs;
use std::io;
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