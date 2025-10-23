use ratatui::{buffer::Buffer, layout::Rect};

pub trait Page {
    fn render(&self, area: Rect, buf: &mut Buffer);
}

#[derive(Default)]
pub struct Navigator {
    pub pages: Vec<Box<dyn Page>>,
    pub page_counter: usize,
}

impl Navigator {
    pub fn push(&mut self, page: Box<dyn Page>) {
        self.pages.push(page);
        self.page_counter = self.pages.len() - 1;
    }

    pub fn push_replace(&mut self, page: Box<dyn Page>) {
        self.pages.truncate(self.page_counter);
        self.pages.push(page);
        self.page_counter = self.pages.len() - 1;
    }

    pub fn pop(&mut self) -> Option<Box<dyn Page>> {
        let page = self.pages.pop();
        self.page_counter = self.pages.len() - 1;
        page
    }

    pub fn current_page(&self) -> Option<&Box<dyn Page>> {
        self.pages.get(self.page_counter)
    }

    pub fn next(&mut self) {
        if self.page_counter == self.pages.len() - 1 {
            return;
        }
        self.page_counter = (self.page_counter + 1) % self.pages.len();
    }

    pub fn previous(&mut self) {
        if self.page_counter == 0 {
            return;
        }
        self.page_counter = (self.page_counter - 1) % self.pages.len();
    }
}