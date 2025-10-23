use ratatui::{
    buffer::Buffer, layout::{Alignment, Constraint, Direction, Flex, Layout, Rect}, style::{Color, Style, Stylize}, symbols::border, text::{Line, Text}, widgets::{Block, Padding, Paragraph, Widget}
};

use crate::{layout::{get_default_style, get_instructions, set_title}, navigator::Page};

#[derive(Debug, Default)]
pub struct HelloPage;

impl HelloPage {
    fn center_rect(&self, area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
        let [area] = Layout::horizontal([horizontal]).flex(Flex::Center).areas(area);
        let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
        area
    }
}

impl Page for HelloPage {

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let title = set_title(Some(" Hello ")).style(
            get_default_style())
            .centered();
        let instructions = get_instructions().style(
            get_default_style())
            .centered();
        let block = Block::new()
            .title(title)
            .title_bottom(instructions)
            .style(get_default_style());
        block.render(area, buf);
        let content = Text::from(vec![
            "Hello, world!".bold().into(),
            "This is a test of the Paragraph widget.".into(),
            "It should be centered and styled.".into(),
        ]).style(get_default_style()).centered();
        let content_area = self.center_rect(area, Constraint::Length(content.width() as u16), Constraint::Length(content.height() as u16));
        Paragraph::new(content).render(content_area, buf);
    }
}

impl Widget for HelloPage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render(area, buf);
    }
}