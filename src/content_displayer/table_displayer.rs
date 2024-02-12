use chrono::{DateTime, Utc};
use tabled::{
    settings::{
        height::CellHeightLimit,
        peaker::PriorityMax,
        style::On,
        width::{MinWidth, Wrap},
        Height as TabledHeight, Settings, Style, Width as TabledWidth,
    },
    Table, Tabled,
};
use terminal_size::{terminal_size, Height as TerminalHeight, Width as TerminalWidth};

use crate::error::folio_error::FolioError;

const MAX_TABLE_CELL_STRING_VEC_ELEMENTS: usize = 3;

pub fn build_table<I: Tabled>(content: &Vec<I>) -> Result<Table, FolioError> {
    let settings: FolioTableSettings = build_table_settings()?;
    Ok(Table::new(content).with(settings).clone())
}

pub fn display_firsts_string_vec(vec: &Vec<String>) -> String {
    let mut string_vec = String::new();
    for (i, value) in vec.iter().enumerate() {
        if i > 0 {
            string_vec.push_str(", ");
        }
        string_vec.push_str(value);
        if i == MAX_TABLE_CELL_STRING_VEC_ELEMENTS - 1 {
            string_vec.push_str(format!(", {} more...", vec.len() - i).as_str());
            break;
        }
    }
    string_vec
}

pub fn display_count(to_count: &Vec<String>) -> String {
    to_count.len().to_string()
}

pub fn display_written_length(possible_description: &Option<String>) -> String {
    match possible_description {
        None => "Not written ❌".to_string(),
        Some(description) => format!("Written ✅ ({} chars)", description.len()),
    }
}

pub fn display_date(date: &DateTime<Utc>) -> String {
    format!("{}", date.date_naive())
}

type FolioTableSettings = Settings<
    Settings<
        Settings<
            Settings<
                Settings<Settings, tabled::settings::Style<On, On, On, On, (), On, 1, 0>>,
                tabled::settings::Style<On, On, On, On, (), On, 1, 0>,
            >,
            Wrap<usize, PriorityMax>,
        >,
        MinWidth,
    >,
    CellHeightLimit,
>;

fn build_table_settings() -> Result<FolioTableSettings, FolioError> {
    let (width, height) = get_terminal_size()?;
    let settings = Settings::default()
        .with(Style::rounded())
        .with(Style::rounded())
        .with(TabledWidth::wrap(width).priority::<PriorityMax>())
        .with(TabledWidth::increase(width))
        .with(TabledHeight::limit(height));
    Ok(settings)
}

fn get_terminal_size() -> Result<(usize, usize), FolioError> {
    match terminal_size() {
        None => Err(FolioError {
            message: "Unable to get terminal size".to_string(),
        }),
        Some((TerminalWidth(w), TerminalHeight(h))) => Ok((w as usize, h as usize)),
    }
}
