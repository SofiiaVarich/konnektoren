use crate::model::{AdjectiveType, KonnektorType, TestType, VerbType};
use ratatui::{
    prelude::*,
    widgets::{block::*, Borders, Wrap, *},
};
use strum::IntoEnumIterator;

pub struct TypeButtons {
    test_type: TestType,
}

impl TypeButtons {
    pub fn new(test_type: TestType) -> Self {
        Self { test_type }
    }
}

impl Widget for TypeButtons {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::ALL);
        let inner_area = block.inner(area);

        let instructions: Vec<Line> = match self.test_type {
            TestType::Konnektoren => KonnektorType::iter()
                .enumerate()
                .map(|(index, konnektor_type)| {
                    Line::raw(format!("{}({})\n", konnektor_type, index + 1))
                })
                .collect(),
            TestType::Adjectives => AdjectiveType::iter()
                .enumerate()
                .map(|(index, adjective_type)| {
                    Line::raw(format!("{}({})\n", adjective_type, index + 1))
                })
                .collect(),
            TestType::Verbs => VerbType::iter()
                .enumerate()
                .map(|(index, verb_type)| Line::raw(format!("{}({})\n", verb_type, index + 1)))
                .collect(),
        };

        let paragraph = Paragraph::new(instructions).wrap(Wrap { trim: true });
        paragraph.render(inner_area, buf);
    }
}
