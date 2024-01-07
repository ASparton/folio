use std::env;

use crate::error::missing_gh_token_error::MissingGHTokenError;

const GH_TOKEN_ENV_VAR_NAME: &str = "FOLIO_TOKEN";

pub fn get_gh_token(input: String) -> Result<String, MissingGHTokenError> {
    let possible_token = extract_gh_token_from_input(input);
    if possible_token.is_ok() {
        return possible_token;
    }
    extract_gh_token_from_env()
}

fn extract_gh_token_from_input(input: String) -> Result<String, MissingGHTokenError> {
    let words: Vec<&str> = input.split(' ').collect();
    for (i, word) in words.iter().enumerate() {
        if (*word == "-t" || *word == "--token") && i + 1 < words.len() {
            return Ok(words[i + 1].to_string());
        }
    }
    return Err(MissingGHTokenError);
}

fn extract_gh_token_from_env() -> Result<String, MissingGHTokenError> {
    match env::var(GH_TOKEN_ENV_VAR_NAME) {
        Err(_error) => Err(MissingGHTokenError),
        Ok(token) => Ok(token),
    }
}
