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

pub fn build_table<I: Tabled>(content: &Vec<I>) -> Result<Table, FolioError> {
    let settings = build_table_settings()?;
    Ok(Table::new(content).with(settings).clone())
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
