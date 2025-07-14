mod commands;
mod parse;
use commands::*;
use parse::*;
use std::io::{self, Write};
fn main() {
    let mut curr = match std::env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(_) => String::new(),
    };
    loop {
        if let Ok(p) = std::env::current_dir() {
            curr = p.display().to_string();
        }

        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let bytes = io::stdin().read_line(&mut input).unwrap();
        if bytes == 0 {
            break;
        }
        let mut input = clean_input(&input);
        if input.is_empty() || input == "\n" {
            continue;
        }

        let cmd = loop {
            if let Ok(c) = split(input.trim()) {
                break c;
            }
            print!("dequote> ");
            io::stdout().flush().unwrap();
            let mut cont = String::new();
            let bytes = io::stdin().read_line(&mut cont).unwrap();
            if bytes == 0 {
                return;
            }
            input.push_str(&cont);
        };
        match cmd.command.as_str() {
            "exit" => break,
            "echo" => echo::echo(input.trim()),
            "pwd" => pwd::pwd(&curr),
            "mkdir" => mkdir::mkdir(cmd.args),
            "cat" => cat::cat(cmd.args),
            "cp" => cp::cp(cmd.args),
            "cd" => cd::cd(cmd.args),
            "mv" => mv::mv(cmd.args),
            "rm" => rm::rm(cmd.args),
            "ls" => match ls::ls(cmd.args){
                Ok(())=>{}
                Err(e)=>{
                    println!("{}",e)
                }
            },
            "clear" => clear::clear(cmd.args),
            _ => println!("Command '{}' not found", cmd.command),
        }
    }
}
