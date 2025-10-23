use ratatui::text::Line;
use ratatui::style::{Color, Style, Stylize};

const DEFAULT_TITLE: &str = " Mnstr v2 CLI ";

pub fn set_title<'a>(text: Option<&'a str>) -> Line<'a> {
    Line::from(text.unwrap_or(DEFAULT_TITLE).bold())
}

pub fn get_instructions<'a>() -> Line<'a> {
    Line::from(vec![
        "esc".bold(), " to quit, ".into(),
        "shift + left/right".bold(), " previous/next page, ".into(),
        "enter".bold(), " select, ".into(),
        "tab/shift+tab".bold(), " cycle. ".into(),
    ])
}

pub fn get_default_style() -> Style {
    Style::default()
    .fg(Color::Black)
    .bg(Color::Rgb(0, 240, 255))
}