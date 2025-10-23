use std::fmt::Debug;
use ratatui::{buffer::Buffer, layout::Rect};

pub trait Page: Debug {
    fn render(&self, area: Rect, buf: &mut Buffer);
}

