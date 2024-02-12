use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;
use std::fmt::Display;

use crate::error::folio_error::FolioError;

pub fn make_selection<T: Display>(options: &[T], question: &str) -> Result<usize, FolioError> {
    match FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("{}", question))
        .default(0)
        .items(options)
        .interact_opt()?
    {
        None => return Err(FolioError::empty_new()),
        Some(index) => Ok(index),
    }
}

pub fn ask_for_confirmation(question: &str) -> Result<bool, FolioError> {
    match dialoguer::Confirm::new()
        .with_prompt(format!("{}", question))
        .interact_opt()?
    {
        None => return Err(FolioError::empty_new()),
        Some(index) => Ok(index),
    }
}
