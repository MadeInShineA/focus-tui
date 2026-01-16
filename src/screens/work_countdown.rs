use crate::app::{Action, Screen};
use ratatui::{crossterm::event::Event, style::Style, text::Text, widgets::Paragraph};

pub struct WorkCountdownScreen {
    pub initial_countdown_seconds: i32,
    pub elapsed_time_seconds: i32,
}

impl Screen for WorkCountdownScreen {
    fn draw(&self, frame: &mut ratatui::Frame) {
        let countdown_title: Text = Text::styled("Countdown screen!", Style::default()).centered();

        let countdown_title_paragraph: Paragraph = Paragraph::new(countdown_title).centered();

        frame.render_widget(countdown_title_paragraph, frame.area());
    }
    fn handle_events(&mut self, event: &Event) -> Option<Action> {
        None
    }

    fn update(&mut self) {}
}
