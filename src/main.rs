mod commands;
use commands::*;
use std::env;
use std::io::{self, Write};
use std::process;
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
        let input = clean_input(input.trim());
        if input.is_empty() {
            continue;
        }

        let cmd = split(input);
        match cmd.command.as_str() {
            "exit" => break,
            "echo" => echo::echo(cmd),
            "pwd" => pwd::pwd(),
            "mkdir" => mkdir::mkdir(cmd.args),
            "cat" => cat::cat(cmd.args),
            "cp" => cp::cp(cmd.args),
            "cd" => cd::cd(cmd.args),
            _ => println!("Command '{}' not found", cmd.command),
        }
    }
    println!("good bye, we wont miss you.");
}
pub struct Cmd {
    pub command: String,
    pub args: Vec<String>,
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
    let mut var = String::new();
    while let Some(&ch) = chars.peek() {
        match ch {
            '"' => {
                chars.next(); // skip the quote
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {
                chars.next(); // skip the space
                if !var.is_empty() {
                    match env::var_os(var.trim()) {
                        Some(r) => {
                            if let Ok(r) = r.into_string() {
                                current.push_str(&r)
                            }
                        }
                        _ => {}
                    }
                    var.clear();
                }
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            '$' => {
                if var.is_empty() {
                    var.push(' ');
                } else {
                    current.push_str(&process::id().to_string());
                    var.clear();
                }
                chars.next();
            }
            _ => {
                if var.is_empty() {
                    current.push(ch);
                } else if ch.is_alphanumeric() {
                    var.push(ch)
                } else {
                    match env::var_os(var.trim()) {
                        Some(r) => {
                            if let Ok(r) = r.into_string() {
                                current.push_str(&r)
                            }
                        }
                        _ => {}
                    }
                    var.clear();
                    current.push(ch);
                }
                chars.next();
            }
        }
    }
    if !var.is_empty() {
        match env::var_os(var.trim()) {
            Some(r) => {
                if let Ok(r) = r.into_string() {
                    current.push_str(&r)
                }
            }
            _ => {}
        }
        var.clear();
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    let command = match tokens.get(0) {
        Some(cmd) => cmd.clone(),
        _ => {
            return Cmd {
                command: String::new(),
                args: vec![],
            }
        }
    };

    let mut args = Vec::new();

    for token in tokens.iter().skip(1) {
        args.push(token.clone());
    }
    Cmd { command, args }
}
