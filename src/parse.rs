use std::env;
/// Represents a parsed command from user input.
#[derive(Debug)]
pub struct Cmd {
    /// The command name (e.g., "ls", "echo")
    pub command: String,
    /// The arguments passed to the command (excluding the command itself)
    pub args: Vec<String>,
}

/// Removes ANSI escape sequences used for cursor movement from the input.
///
/// # Example
/// ```
/// let input = "\x1B[A";
/// let cleaned = clean_input(input);
/// assert_eq!(cleaned, "");
/// ```
pub fn clean_input(input: &str) -> String {
    let re = regex::Regex::new(r"\x1B\[[A-D]").unwrap();
    re.replace_all(input, "").to_string()
}
/// Parses a user input string into a structured `Cmd` object.
///
/// Supports:
/// - Single and double quoting
/// - Environment variable expansion: `$VAR`, `${VAR}`
/// - Home directory expansion: `~`
///
/// # Errors
///
/// Returns `Err(1)` if there's an unmatched quote.
///
/// # Example
/// ```
/// let cmd = split("echo \"$HOME\"").unwrap();
/// assert_eq!(cmd.command, "echo");
/// assert_eq!(cmd.args, vec![std::env::var("HOME").unwrap()]);
/// ```
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
                chars.next();
                let mut var_name = String::new();
                if let Some(&'{') = chars.peek() {
                    chars.next(); 
                    while let Some(&c) = chars.peek() {
                        if c == '}' {
                            chars.next(); 
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