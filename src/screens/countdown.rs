use std::ops::ControlFlow::Break;
use std::time::{Duration, Instant};

use crate::app::{Action, Screen};
use crate::theme::Theme;
use crate::utils::popup_area;
use notify_rust::Notification;
use ratatui::widgets::Clear;
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, Gauge, Paragraph},
};
use tui_big_text::{BigText, PixelSize};

pub enum CountdownType {
    Work,
    Break,
}

impl CountdownType {
    fn next(&self) -> CountdownType {
        match self {
            CountdownType::Work => CountdownType::Break,
            CountdownType::Break => CountdownType::Work,
        }
    }
}

pub struct CountdownScreen {
    start_time: Instant,
    total_duration: Duration,
    remaining_time_when_paused: Option<Duration>,
    work_duration: Duration,
    break_duration: Duration,
    countdown_type: CountdownType,
}

impl CountdownScreen {
    pub fn new(work_duration_minutes: u64, break_duration_minutes: u64) -> Self {
        CountdownScreen {
            start_time: Instant::now(),
            total_duration: Duration::from_mins(work_duration_minutes),
            remaining_time_when_paused: None,
            work_duration: Duration::from_mins(work_duration_minutes),

            break_duration: Duration::from_mins(break_duration_minutes),

            countdown_type: CountdownType::Work,
        }
    }

    fn is_paused(&self) -> bool {
        self.remaining_time_when_paused.is_some()
    }

    fn remaining_duration(&self) -> Duration {
        if let Some(remaining_time) = self.remaining_time_when_paused {
            remaining_time
        } else {
            self.total_duration
                .saturating_sub(self.start_time.elapsed())
        }
    }

    // TODO Maybe define a function inside the CountdownType ?
    fn end_countdown(&mut self) {
        self.countdown_type = self.countdown_type.next();
        self.total_duration = match self.countdown_type {
            CountdownType::Work => self.work_duration,
            CountdownType::Break => self.break_duration,
        };

        self.start_time = Instant::now();
        // Stay paused if we were paused, with the full duration of the new countdown
        if self.is_paused() {
            self.remaining_time_when_paused = Some(self.total_duration);
        }
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) {
        match key_event.code {
            KeyCode::Char(' ') => match self.remaining_time_when_paused.take() {
                // Resume: shift start_time so the elapsed time matches what it was at pause
                Some(remaining_time) => {
                    let elapsed_before_pause = self.total_duration.saturating_sub(remaining_time);
                    self.start_time = Instant::now() - elapsed_before_pause;
                }
                // Pause: freeze the remaining time
                None => {
                    self.remaining_time_when_paused = Some(
                        self.total_duration
                            .saturating_sub(self.start_time.elapsed()),
                    );
                }
            },
            KeyCode::Char('s') => {
                self.end_countdown();
            }
            _ => {}
        }
    }

    fn render_pause(&self, frame: &mut ratatui::Frame, area: Rect, theme: &Theme) {
        let block = Block::bordered()
            .border_style(theme.border_style)
            .style(theme.background_style);
        let popup_area = popup_area(area, 60, 20);
        frame.render_widget(Clear, popup_area);
        let inner_area = block.inner(popup_area);

        let pause_text: Text =
            Text::styled("The countdown is paused!", theme.text_style()).centered();
        let pause_paragraph: Paragraph = Paragraph::new(pause_text).centered();

        frame.render_widget(block, popup_area);
        frame.render_widget(pause_paragraph, inner_area);
    }
}

impl Screen for CountdownScreen {
    fn draw(&self, frame: &mut ratatui::Frame, area: Rect, theme: &Theme) {
        let progress_gauge_label: String = match self.countdown_type {
            CountdownType::Work => String::from("Work countdown"),
            CountdownType::Break => String::from("Break countdown"),
        };

        let progress_gauge_style: Style = theme.gauge_style(&self.countdown_type);

        let progress_gauge_percent: u16 =
            ((self.total_duration.as_secs() - self.remaining_duration().as_secs()) * 100
                / self.total_duration.as_secs()) as u16;

        let progress_gauge: Gauge = Gauge::default()
            .block(Block::bordered().border_style(theme.border_style))
            .label(progress_gauge_label)
            .gauge_style(progress_gauge_style)
            .percent(progress_gauge_percent);

        let countdown_big_text_color: Color = theme.countdown_color(&self.countdown_type);

        let remaining_seconds: u64 = self.remaining_duration().as_secs();
        let hours: u64 = remaining_seconds / 3600;
        let minutes: u64 = remaining_seconds / 60 % 60;
        let seconds: u64 = remaining_seconds % 60;

        let countdown_big_text: BigText = BigText::builder()
            .pixel_size(PixelSize::Full)
            .lines(vec![
                format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
                    .fg(countdown_big_text_color)
                    .into(),
            ])
            .centered()
            .build();

        let controls_text: Text =
            Text::styled("Controls: Space to pause, Q to quit", theme.text_style()).centered();
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
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
            .split(top_area);

        frame.render_widget(progress_gauge, top_layout[0]);
        frame.render_widget(countdown_big_text, top_layout[1]);

        if self.is_paused() {
            self.render_pause(frame, area, theme);
        }
    }

    fn handle_event(&mut self, event: &Event) -> Option<Action> {
        match event {
            Event::Key(key_event) => {
                self.handle_key_event(key_event);
                None
            }

            _ => None,
        }
    }

    fn update(&mut self) {
        if !self.is_paused() && self.remaining_duration().as_secs() == 0 {
            self.end_countdown();

            let notification = match self.countdown_type {
                CountdownType::Work => Notification::new()
                    .summary("Work time started")
                    .body("The work countdown has started, please focus!")
                    .finalize(),
                CountdownType::Break => Notification::new()
                    .summary("Break time started")
                    .body("The break countdown has started, please take some time to relax!")
                    .finalize(),
            };

            if let Err(e) = notification.show() {
                eprintln!("Failed to show notification: {}", e);
            }
        }
    }
}
