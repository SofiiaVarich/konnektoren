use crate::model::TestType;
use crate::tui::components::TypeButtons;
use ratatui::{
    prelude::*,
    style::palette::tailwind,
    widgets::{block::*, Borders, *},
};

pub struct Tab {
    pub test_type: TestType,
}

impl Tab {
    pub fn new(test_type: TestType) -> Self {
        Self { test_type }
    }

    pub fn title(&self) -> Line<'static> {
        format!("  {}  ", self.test_type)
            .fg(tailwind::SLATE.c200)
            .into()
    }

    pub fn block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
    }

    pub fn palette(&self) -> tailwind::Palette {
        match self.test_type {
            TestType::Konnektoren => tailwind::BLUE,
            TestType::Adjectives => tailwind::EMERALD,
            TestType::Verbs => tailwind::INDIGO,
        }
    }
}

impl Widget for Tab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.title())
            .block(self.block())
            .render(area, buf);
        let block = Block::default().borders(Borders::ALL);
        let inner_area = block.inner(area);

        TypeButtons::new(self.test_type).render(inner_area, buf);
    }
}
