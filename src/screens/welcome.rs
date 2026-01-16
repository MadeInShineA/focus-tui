use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Text,
    widgets::Paragraph,
};

use crate::app::{Action, Screen};

enum SelectedDuration {
    Work,
    Break,
}

pub struct WelcomeScreen {
    work_duration_minutes: u64,
    break_duration_minutes: u64,
    selected_duration: SelectedDuration,
}

impl WelcomeScreen {
    pub fn new() -> Self {
        WelcomeScreen {
            work_duration_minutes: 45,
            break_duration_minutes: 10,
            selected_duration: SelectedDuration::Work,
        }
    }

    fn increase_work_duration(&mut self) {
        self.work_duration_minutes = self.work_duration_minutes.saturating_add(1);
    }

    fn decrease_work_duration(&mut self) {
        self.work_duration_minutes = (self.work_duration_minutes.saturating_sub(1)).max(1);
    }

    fn increase_break_duration(&mut self) {
        self.break_duration_minutes = self.break_duration_minutes.saturating_add(1);
    }
    fn decrease_break_duration(&mut self) {
        self.break_duration_minutes = (self.break_duration_minutes.saturating_sub(1)).max(1);
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) {
        match key_event.code {
            KeyCode::Tab | KeyCode::Up | KeyCode::Down => match self.selected_duration {
                SelectedDuration::Work => self.selected_duration = SelectedDuration::Break,
                SelectedDuration::Break => self.selected_duration = SelectedDuration::Work,
            },
            KeyCode::Left => match self.selected_duration {
                SelectedDuration::Work => self.decrease_work_duration(),
                SelectedDuration::Break => self.decrease_break_duration(),
            },
            KeyCode::Right => match self.selected_duration {
                SelectedDuration::Work => self.increase_work_duration(),
                SelectedDuration::Break => self.increase_break_duration(),
            },
            _ => {}
        }
    }
}

impl Screen for WelcomeScreen {
    fn draw(&self, frame: &mut ratatui::Frame, area: Rect) {
        let welcome_text: Text = Text::styled("Welcome to Focus Tui!", Style::default()).centered();
        let welcome_paragraph: Paragraph = Paragraph::new(welcome_text).centered();

        let work_style: Style = if matches!(self.selected_duration, SelectedDuration::Work) {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        let work_duration_text: Text = Text::styled(
            format!("Work duration: {} min", self.work_duration_minutes),
            work_style,
        )
        .centered();
        let work_duration_paragraph: Paragraph = Paragraph::new(work_duration_text).centered();

        let break_style: Style = if matches!(self.selected_duration, SelectedDuration::Break) {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        let break_duration_text: Text = Text::styled(
            format!("Break duration: {} min", self.break_duration_minutes),
            break_style,
        )
        .centered();
        let break_duration_paragraph: Paragraph = Paragraph::new(break_duration_text).centered();

        let controls_text: Text = Text::styled(
            "Controls: Tab/Up/Down to select duration, Left/Right to change value, Enter to start, Q to quit",
            Style::default(),
        )
        .centered();
        let controls_paragraph: Paragraph = Paragraph::new(controls_text).centered();

        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        let top_area: Rect = vertical_layout[0];
        let bottom_area: Rect = vertical_layout[1];

        frame.render_widget(controls_paragraph, bottom_area);

        let top_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(top_area);

        frame.render_widget(welcome_paragraph, top_layout[0]);
        frame.render_widget(work_duration_paragraph, top_layout[1]);
        frame.render_widget(break_duration_paragraph, top_layout[2]);
    }

    fn handle_event(&mut self, event: &Event) -> Option<Action> {
        match event {
            Event::Key(key_event) if key_event.code == KeyCode::Enter => {
                Some(Action::SetDurations {
                    work_duration_minutes: self.work_duration_minutes,
                    break_duration_minutes: self.break_duration_minutes,
                })
            }

            Event::Key(key_event) => {
                self.handle_key_event(key_event);
                None
            }

            _ => None,
        }
    }

    fn update(&mut self) {}
}
