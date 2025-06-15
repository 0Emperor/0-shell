use crate::Cmd;
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

