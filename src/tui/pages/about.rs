use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub struct AboutPage {
    title: String,
    content: Vec<String>,
}

impl AboutPage {
    fn new(title: impl Into<String>, content: Vec<impl Into<String>>) -> Self {
        Self {
            title: title.into(),
            content: content.into_iter().map(Into::into).collect(),
        }
    }
}

impl Widget for AboutPage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(self.title)
            .title_alignment(Alignment::Center);

        let inner_area = block.inner(area);
        block.render(area, buf);

        let text: Vec<Line> = self
            .content
            .iter()
            .map(|text| Line::raw(text.as_str()))
            .collect();

        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
            .render(inner_area, buf);
    }
}

impl Default for AboutPage {
    fn default() -> Self {
        let content = vec![
            "This platform is dedicated to helping individuals improve their understanding and use of German grammar.",
            "Specifically, you can learn about:",
            "- Konnektoren: Understand how to connect clauses and sentences.",
            "- Adjektive: Dive into the use of adjectives, including those with prepositions.",
            "- Verben: Master the use of verbs, particularly those with prepositions.",
            "Through interactive tests and examples, this platform aims to enhance your German grammar skills.",
        ];

        Self::new(
            "About This Learning Platform",
            content.into_iter().map(String::from).collect(),
        )
    }
}
