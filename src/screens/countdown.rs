use std::time::{Duration, Instant};

use crate::app::{Action, Screen};
use notify_rust::Notification;
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, Gauge, Paragraph},
};
use tui_big_text::{BigText, PixelSize};
enum CountdownType {
    Work,
    Break,
}

pub struct CountdownScreen {
    start_time: Instant,
    total_duration: Duration,
    is_paused: bool,
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
            is_paused: false,
            remaining_time_when_paused: None,
            work_duration: Duration::from_mins(work_duration_minutes),

            break_duration: Duration::from_mins(break_duration_minutes),

            countdown_type: CountdownType::Work,
        }
    }

    fn remaining_duration(&self) -> Duration {
        if let Some(remaining_time) = self.remaining_time_when_paused {
            remaining_time
        } else {
            self.total_duration
                .saturating_sub(self.start_time.elapsed())
        }
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) {
        match key_event.code {
            KeyCode::Char(' ') => {
                if self.is_paused {
                    if let Some(remaining_time) = self.remaining_time_when_paused {
                        let elapsed_during_pause = self.total_duration - remaining_time;
                        self.start_time = Instant::now() - elapsed_during_pause;
                    }
                    self.remaining_time_when_paused = None;
                    self.is_paused = false;
                } else {
                    self.remaining_time_when_paused = Some(
                        self.total_duration
                            .saturating_sub(self.start_time.elapsed()),
                    );
                    self.is_paused = true;
                }
            }
            _ => {}
        }
    }

    fn render_pause(&self, frame: &mut ratatui::Frame, area: Rect) {
        let block = Block::bordered();
        let popup_area = popup_area(area, 60, 20);
        let inner_area = block.inner(popup_area);

        let pause_text: Text =
            Text::styled("The countdown is paused!", Style::default()).centered();
        let pause_paragraph: Paragraph = Paragraph::new(pause_text).centered();

        frame.render_widget(block, popup_area);
        frame.render_widget(pause_paragraph, inner_area);
    }
}

impl Screen for CountdownScreen {
    fn draw(&self, frame: &mut ratatui::Frame, area: Rect) {
        let progress_gauge_label: String = match self.countdown_type {
            CountdownType::Work => String::from("Work countdown"),
            CountdownType::Break => String::from("Break countdown"),
        };

        let progress_gauge_style: Style = match self.countdown_type {
            CountdownType::Work => Style::new()
                .fg(Color::Rgb(166, 227, 161))
                .bg(Color::Rgb(30, 30, 46)),
            CountdownType::Break => Style::new()
                .fg(Color::Rgb(137, 180, 250))
                .bg(Color::Rgb(30, 30, 46)),
        };

        let progress_gauge_percent: u16 =
            ((self.total_duration.as_secs() - self.remaining_duration().as_secs()) * 100
                / self.total_duration.as_secs()) as u16;

        let progress_gauge: Gauge = Gauge::default()
            .block(Block::bordered())
            .label(progress_gauge_label)
            .gauge_style(progress_gauge_style)
            .percent(progress_gauge_percent);

        let countdown_big_text_color: Color = match self.countdown_type {
            CountdownType::Work => Color::Rgb(166, 227, 161),
            CountdownType::Break => Color::Rgb(137, 180, 250),
        };

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
            Text::styled("Controls: Space to pause, Q to quit", Style::default()).centered();
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

        if self.is_paused {
            self.render_pause(frame, area);
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
        if !self.is_paused && self.remaining_duration().as_secs() == 0 {
            self.countdown_type = match self.countdown_type {
                CountdownType::Work => CountdownType::Break,
                CountdownType::Break => CountdownType::Work,
            };

            self.total_duration = match self.countdown_type {
                CountdownType::Work => self.work_duration,
                CountdownType::Break => self.break_duration,
            };

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

            self.start_time = Instant::now();
            self.remaining_time_when_paused = None;
        }
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
