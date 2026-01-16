use ratatui::{
    crossterm::event::{Event, KeyCode},
    style::Style,
    text::Text,
    widgets::Paragraph,
};

use crate::app::{Action, Screen};

pub struct WelcomeScreen;

impl Screen for WelcomeScreen {
    fn draw(&self, frame: &mut ratatui::Frame) {
        let welcome_text: Text = Text::styled("Welcome to Focus Tui!", Style::default()).centered();

        let welcome_paragraph: Paragraph = Paragraph::new(welcome_text).centered();

        frame.render_widget(welcome_paragraph, frame.area());
    }

    fn handle_events(&mut self, event: &Event) -> Option<Action> {
        match event {
            Event::Key(key_event) if key_event.code == KeyCode::Enter => {
                Some(Action::SwitchToWorkCountdownScreen)
            }
            _ => None,
        }
    }

    fn update(&mut self) {}
}
