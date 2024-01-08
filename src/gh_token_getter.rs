use std::env;

use crate::error::missing_gh_token_error::MissingGHTokenError;

const GH_TOKEN_ENV_VAR_NAME: &str = "FOLIO_TOKEN";

pub fn get_gh_token(input_args: &Vec<String>) -> Result<String, MissingGHTokenError> {
    let possible_token = extract_gh_token_from_input(input_args);
    if possible_token.is_ok() {
        return possible_token;
    }
    extract_gh_token_from_env()
}

fn extract_gh_token_from_input(input_args: &Vec<String>) -> Result<String, MissingGHTokenError> {
    for (i, arg) in input_args.iter().enumerate() {
        if (*arg == "-t" || *arg == "--token") && i + 1 < input_args.len() {
            return Ok(input_args[i + 1].to_string());
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
