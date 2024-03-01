use ratatui::{layout::Rect, style::{Color, Style}, widgets::{Block, Borders, Paragraph}};
use tui_textarea::{Input, Key};

use crate::action::Action;

use super::Component;

pub struct Response {
    pub selected: bool,
    pub response_value: String,
}

impl Response {
    pub fn new() -> Self {
        Self {
            selected: false,
            response_value: String::new(),
        }
    }

    pub fn update_response_value(&mut self, response: String) {
        self.response_value = response;
    }
}

impl Component for Response {
    fn handle_key_events(&mut self) -> Option<Action> {
        let event_result = crossterm::event::read();
        match event_result {
            Ok(event) => {
                match event.into() {
                    Input { key: Key::Esc, .. } => self.handle_deselect(),
                    _ => None
                }
            }
            Err(_) => Some(Action::Suspend)
        }
    }


    fn handle_deselect(&mut self) -> Option<Action> {
        self.selected = false;
        Some(Action::Suspend)
    }

    fn handle_select(&mut self) {
        self.selected = true;
    }

    fn render_frame(&mut self, frame: &mut ratatui::prelude::Frame<'_>, area: Rect) -> std::io::Result<()> {
        if self.selected {
            let p = Paragraph::new(self.response_value.as_str())
                        .block(Block::default().title("Response").borders(Borders::ALL).style(Style::default().fg(Color::Red)));
            frame.render_widget(p, area);

        } else {
            let p = Paragraph::new(self.response_value.as_str())
                        .block(Block::default().title("Response").borders(Borders::ALL));
            frame.render_widget(p, area);
        }
        Ok(())
    }
}