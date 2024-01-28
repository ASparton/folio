use std::fmt::Display;
use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;

use crate::error::folio_error::FolioError;

pub fn make_selection<T: Display>(options: &[T], question: &str) -> Result<usize, FolioError> {
    match FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("\nWhich portfolio do you want to see?")
        .default(0)
        .items(options)
        .interact_opt()?
    {
        None => return Err(FolioError::empty_new()),
        Some(index) => Ok(index),
    }
}