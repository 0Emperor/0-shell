mod commands;
use commands::*;
use std::env;
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
        let mut input = clean_input(&input);
        if input.is_empty() {
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
#[derive(Debug)]
pub struct Cmd {
    pub command: String,
    pub args: Vec<String>,
}
fn clean_input(input: &str) -> String {
    let re = regex::Regex::new(r"\x1B\[[A-D]").unwrap();
    re.replace_all(input, "").to_string()
}
pub fn split(input: &str) -> Result<Cmd, u8> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();

    let mut in_single_quotes = false;
    let mut in_double_quotes = false;

    while let Some(&ch) = chars.peek() {
        match ch {
            '\'' if !in_double_quotes => {
                chars.next();
                in_single_quotes = !in_single_quotes;
            }
            '"' if !in_single_quotes => {
                chars.next();
                in_double_quotes = !in_double_quotes;
            }
            ' ' if !in_single_quotes && !in_double_quotes => {
                chars.next();
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            '$' if !in_single_quotes => {
                chars.next(); // skip '$'
                let mut var_name = String::new();
                if let Some(&'{') = chars.peek() {
                    chars.next(); // skip '{'
                    while let Some(&c) = chars.peek() {
                        if c == '}' {
                            chars.next(); // skip '}'
                            break;
                        }
                        var_name.push(c);
                        chars.next();
                    }
                } else {
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            var_name.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }
                if let Ok(val) = env::var(&var_name) {
                    current.push_str(&val);
                }
            }
            '~' if current.is_empty() && !in_single_quotes && !in_double_quotes => {
                chars.next();
                if let Ok(home) = env::var("HOME") {
                    current.push_str(&home);
                } else {
                    current.push('~');
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

    let command = tokens.get(0).cloned().unwrap_or_default();
    let args = tokens.iter().skip(1).cloned().collect();
    if in_double_quotes || in_single_quotes {
        return Err(1);
    }
    Ok(Cmd { command, args })
}
