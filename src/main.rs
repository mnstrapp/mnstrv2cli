use std::io;

use crate::app::App;

mod app;
mod pages;
mod navigator;
mod layout;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
