use std::io;

use crate::tui::components::Tab;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*};
use strum::IntoEnumIterator;

use crate::model::TestType;
use crate::tui;
use crate::tui::AboutPage;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    About,
    Quitting,
}

#[derive(Default)]
pub struct App {
    test_type: TestType,
    state: AppState,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while self.state != AppState::Quitting {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = TestType::iter().map(|t| Tab::new(t).title());
        let tab = Tab::new(self.test_type);
        let highlight_style = (Color::default(), tab.palette().c700);
        let selected_tab_index = self.test_type as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('a') => self.about(),
            KeyCode::Char('t') => self.running(),
            KeyCode::Esc => self.running(),
            KeyCode::Tab => self.next_tab(),
            _ => {}
        }
    }

    fn about(&mut self) {
        self.state = AppState::About;
    }

    fn running(&mut self) {
        self.state = AppState::Running;
    }

    fn exit(&mut self) {
        self.state = AppState::Quitting;
    }

    pub fn next_tab(&mut self) {
        self.test_type = match self.test_type {
            TestType::Konnektoren => TestType::Adjectives,
            TestType::Adjectives => TestType::Verbs,
            TestType::Verbs => TestType::Konnektoren,
        };
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::*;
        let title = Text::from(" Konnektoren TUI ".bold());

        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        match self.state {
            AppState::About => {
                AboutPage::default().render(inner_area, buf);
            }
            AppState::Running => {
                self.render_tabs(tabs_area, buf);
                let tab = Tab::new(self.test_type);
                tab.render(inner_area, buf);
            }
            AppState::Quitting => {
                let message = Text::from("Goodbye!".bold());
                Paragraph::new(message).centered().render(inner_area, buf);
            }
        }

        let instructions = Text::from(vec![Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
            " About ".into(),
            "<A> ".blue().bold(),
            " Test ".into(),
            "<T> ".blue().bold(),
            " Next Test ".into(),
            "<Tab>".blue().bold(),
        ])]);

        Paragraph::new(title).centered().render(title_area, buf);

        Paragraph::new(instructions)
            .centered()
            .render(footer_area, buf);
    }
}
