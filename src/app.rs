use std::io;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    buffer::Buffer, layout::Rect, style::Stylize, symbols::border, text::Line, widgets::{Block, Widget}, DefaultTerminal, Frame
};

use crate::{navigator::Navigator, pages::hello::HelloPage};

pub struct App {
    pub exit: bool,
    pub navigator: Navigator,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|f| {
                self.draw(f);
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key) if key.code == KeyCode::Esc => {
                self.exit = true;
            }
            Event::Key(key) if key.modifiers.contains(KeyModifiers::SHIFT) && key.code == KeyCode::Left => {
                self.navigator.previous();
            }
            Event::Key(key) if key.modifiers.contains(KeyModifiers::SHIFT) && key.code == KeyCode::Right => {
                self.navigator.next();
            }
            _ => {}
        }
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        let mut navigator = Navigator::default();
        navigator.push(Box::new(HelloPage::default()));
        Self { exit: false, navigator }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(page) = self.navigator.current_page() {
            page.as_ref().render(area, buf);
        }
    }
}