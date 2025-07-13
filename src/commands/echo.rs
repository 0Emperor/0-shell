use crate::Cmd;
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
pub fn echo(cmd: Cmd) {
    for (arg_index, arg) in cmd.args.iter().enumerate() {
        let mut chars = arg.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.peek() {
                    Some('n') => {
                        print!("\n");
                        chars.next();
                    }
                    Some('t') => {
                        print!("\t");
                        chars.next();
                    }
                    Some('r') => {
                        print!("\r");
                        chars.next();
                    }
                    Some('\\') => {
                        print!("\\");
                        chars.next();
                    }
                    Some('"') => {
                        print!("\"");
                        chars.next();
                    }
                    Some('\'') => {
                        print!("'");
                        chars.next();
                    }
                    Some('c') => {
                        // cancel further output
                        return;
                    }
                    Some(other) => {
                        // unknown escape, print as-is
                        print!("\\{}", other);
                        chars.next();
                    }
                    None => {
                        // lone backslash at end
                        print!("\\");
                    }
                }
            } else {
                print!("{}", c);
            }
        }

        if arg_index < cmd.args.len() - 1 {
            print!(" ");
        }
    }

    println!();
}

