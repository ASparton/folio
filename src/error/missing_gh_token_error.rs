use std::fmt::Display;

/// Raised when an error occured when parsing a string into a command.
#[derive(Debug, Clone)]
pub struct MissingGHTokenError;

impl Display for MissingGHTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut error_message = "Github authentication token is missing. Here are different ways to provide the token:\n".to_string();
        error_message.push_str("- Set the token inside the `FOLIO_TOKEN` environment variable\n");
        error_message.push_str("- Provide the token as command argument using `-t` or `--token`");
        write!(f, "{}", error_message)
    }
}
