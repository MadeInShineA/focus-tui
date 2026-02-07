use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::{Constraint, Layout, Rect},
    text::Text,
    widgets::{Block, Clear, Paragraph},
};

use crate::{
    app::{Action, Popup},
    utils::popup_area,
};

pub struct ErrorPopup {
    error_content: String,
}

impl ErrorPopup {
    pub fn new(error_content: String) -> Self {
        ErrorPopup { error_content }
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) -> Option<Action> {
        match key_event.code {
            KeyCode::Esc => return Some(Action::ClosePopup),
            _ => None,
        }
    }
}

impl Popup for ErrorPopup {
    fn draw(
        &mut self,
        frame: &mut ratatui::Frame,
        area: ratatui::prelude::Rect,
        theme: &crate::theme::Theme,
    ) {
        let block = Block::bordered()
            .border_style(theme.border_style)
            .style(theme.background_style);

        let popup_area: Rect = popup_area(area, 60, 60);
        frame.render_widget(Clear, popup_area);
        let inner_area: Rect = block.inner(popup_area);
        let inner_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![Constraint::Length(1), Constraint::Fill(1)])
            .split(inner_area);

        let title_text: Text = Text::styled("Error popup", theme.text_style()).centered();
        let title_paragraphe: Paragraph = Paragraph::new(title_text).centered();

        let content_text: Text =
            Text::styled(&self.error_content, theme.error_text_style()).centered();

        frame.render_widget(block, popup_area);
        frame.render_widget(title_paragraphe, inner_layout[0]);
        frame.render_widget(content_text, inner_layout[1]);
    }

    fn handle_event(&mut self, event: &Event) -> Option<Action> {
        match event {
            Event::Key(key_event) => self.handle_key_event(key_event),

            _ => None,
        }
    }
}
