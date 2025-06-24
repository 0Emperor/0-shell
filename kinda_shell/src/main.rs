mod commands;
use commands::*;

use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let bytes = io::stdin().read_line(&mut input).unwrap();
        if bytes == 0 {
            println!("ur not worthy anyway...");
            break;
        }
let input=input.trim();
        if input.is_empty() {
            continue;
        }

        let cmd  =split(clean_input(input));
        match cmd.command.as_str() {
            "exit" => break,
            "echo"=> echo::echo(cmd),
            "pwd"=> pwd::pwd(),
            "mkdir"=> mkdir::mkdir(cmd.args),
            "cat"=> cat::cat(cmd.args),
            "cp"=> cp::cp(cmd.args),
            "cd"=> cd::cd(cmd.args),
            "ls"=>ls::ls(cmd.args),
            _ => println!("Command '{}' not found", cmd.command),
        }
    }
    println!("good bye, we wont miss you.");
}
pub struct Cmd {
  pub  command: String,
  pub  args: Vec<String>,
}
fn clean_input(input: &str) -> String {
    let re = regex::Regex::new(r"\x1B\[[A-D]").unwrap();
    re.replace_all(input, "").to_string()
}
fn split(input: String) -> Cmd {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = input.trim().chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '"' => {
                chars.next(); // skip the quote
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {
                chars.next(); // skip the space
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(ch);
                chars.next();
            }
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    let command = match tokens.get(0) {
        Some(cmd) => cmd.clone(),
        None => return Cmd {
            command: String::new(),
            args: vec![],
        },
    };

    let mut args = Vec::new();

    for token in tokens.iter().skip(1) {
        
            args.push(token.clone());
    }

    Cmd {
        command,
        args,
    }
}
