/// Prints the arguments to stdout with escape sequence interpretation.
///
/// Supported escape sequences:
/// - `\n`: newline
/// - `\t`: tab
/// - `\r`: carriage return
/// - `\\`: backslash
/// - `\"`: double quote
/// - `\'`: single quote
/// - `\c`: stop output immediately
///
/// # Arguments
///
/// * `cmd` - Cmd struct containing the command name and arguments.
///
/// # Example
///
/// ```
/// let cmd = Cmd {
///     command: "echo".to_string(),
///     args: vec!["hello\\nworld".to_string()],
/// };
/// echo(cmd);
/// ```
use std::io::{self, Write};

pub fn echo(raw_input: &str) {
    // --- 1. Shell-level parsing ---
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_quotes: Option<char> = None;

    let mut chars = raw_input.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' if in_quotes.is_none() => {
                if let Some(next_char) = chars.next() {
                    current_arg.push(next_char);
                } else {
                    current_arg.push('\\');
                }
            }
            q @ ('"' | '\'') => {
                if in_quotes == Some(q) {
                    in_quotes = None;
                } else if in_quotes.is_none() {
                    in_quotes = Some(q);
                } else {
                    current_arg.push(c);
                }
            }
            ' ' if in_quotes.is_none() => {
                if !current_arg.is_empty() {
                    args.push(current_arg);
                    current_arg = String::new();
                }
            }
            _ => {
                current_arg.push(c);
            }
        }
    }
    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    // --- 2. Interpret the string ---
    let content = args.iter().skip(1).cloned().collect::<Vec<_>>().join(" ");
    let mut chars = content.chars().peekable();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.peek() {
                Some('x') => {
                    chars.next(); // consume 'x'
                    let hi = chars.next();
                    let lo = chars.next();
                    if let (Some(h), Some(l)) = (hi, lo) {
                        if let Ok(byte) = u8::from_str_radix(&format!("{}{}", h, l), 16) {
                            handle.write_all(&[byte]).unwrap();
                            continue;
                        }
                    }
                    // Fallback if malformed
                    handle.write_all(b"").unwrap();
                    if let Some(h) = hi {
                        handle.write_all(&[h as u8]).unwrap();
                    }
                    if let Some(l) = lo {
                        handle.write_all(&[l as u8]).unwrap();
                    }
                }
                Some('n') => {
                    chars.next();
                    handle.write_all(b"\n").unwrap();
                }
                Some('t') => {
                    chars.next();
                    handle.write_all(b"\t").unwrap();
                }
                Some('r') => {
                    chars.next();
                    handle.write_all(b"\r").unwrap();
                }
                Some('c') => return, // stop output
                Some('\\') => {
                    chars.next();
                    handle.write_all(b"\\").unwrap();
                }
                Some(other) => {
                    handle.write_all(&[b'\\', *other as u8]).unwrap();
                }
                None => {
                    handle.write_all(b"\\").unwrap();
                }
            }
        } else {
            handle.write_all(&[c as u8]).unwrap();
        }
    }

    handle.write_all(b"\n").unwrap();
}
