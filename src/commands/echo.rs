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
pub fn echo(raw_input: &str) {
    // --- 1. Shell-level parsing ---
    // This parser now handles unquoted backslashes differently from quoted ones.
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_quotes: Option<char> = None;

    let mut chars = raw_input.chars();
    while let Some(c) = chars.next() {
        match c {
            // NEW: Handle unquoted backslashes for shell-level escaping.
            '\\' if in_quotes.is_none() => {
                // The backslash escapes the next character for the shell.
                // We consume the backslash and append the next character directly.
                if let Some(next_char) = chars.next() {
                    current_arg.push(next_char);
                } else {
                    // A lone trailing backslash is treated literally.
                    current_arg.push('\\');
                }
            }

            // Handle quotes to group arguments.
            q @ ('"' | '\'') => {
                if in_quotes == Some(q) {
                    in_quotes = None; // Closing quote
                } else if in_quotes.is_none() {
                    in_quotes = Some(q); // Opening quote
                } else {
                    current_arg.push(c); // A quote inside another quote type
                }
            }

            // A space outside of quotes is an argument separator.
            ' ' if in_quotes.is_none() => {
                if !current_arg.is_empty() {
                    args.push(current_arg);
                    current_arg = String::new();
                }
            }

            // Any other character is part of the current argument.
            _ => {
                current_arg.push(c);
            }
        }
    }
    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    // Discard the "echo" command name and join the rest.
    let content = args
        .iter()
        .skip(1)
        .cloned()
        .collect::<Vec<String>>()
        .join(" ");

    // --- 2. Echo command execution ---
    // This part remains the same, interpreting the backslashes it receives.
    let mut chars_to_print = content.chars();
    while let Some(c) = chars_to_print.next() {
        if c == '\\' {
            match chars_to_print.next() {
                Some('n') => print!("\n"),
                Some('t') => print!("\t"),
                Some('r') => print!("\r"),
                Some('c') => return,
                Some('\\') => print!("\\"),
                Some(other) => print!("\\{}", other),
                None => print!("\\"),
            }
        } else {
            print!("{}", c);
        }
    }

    println!();
}
