use std::{io, time::Duration};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, poll},
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
};

use crate::screens::{countdown::CountdownScreen, welcome::WelcomeScreen};

pub enum Action {
    Quit,
    SetDurations {
        work_duration_minutes: u64,
        break_duration_minutes: u64,
    },
}
pub trait Screen {
    fn draw(&self, frame: &mut Frame, area: Rect);
    fn handle_event(&mut self, event: &Event) -> Option<Action>;
    fn update(&mut self);
}

// TODO: Use dyn ?
enum CurrentScreen {
    Welcome(WelcomeScreen),
    Countdown(CountdownScreen),
}

pub struct App {
    current_screen: CurrentScreen,
    work_duration_minutes: u64,
    break_duration_minutes: u64,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Welcome(WelcomeScreen::new()),
            work_duration_minutes: 45,
            break_duration_minutes: 10,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            match &mut self.current_screen {
                CurrentScreen::Welcome(screen) => screen.update(),
                CurrentScreen::Countdown(screen) => screen.update(),
            }
            if poll(Duration::from_millis(10))? {
                self.handle_event()?;
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));
        frame.render_widget(block.clone(), frame.area());
        let inner_area = block.inner(frame.area());
        match &self.current_screen {
            CurrentScreen::Welcome(screen) => screen.draw(frame, inner_area),
            CurrentScreen::Countdown(screen) => screen.draw(frame, inner_area),
        }
    }

    fn handle_event(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => self.handle_action(Action::Quit),
                    _ => {
                        if let Some(action) = match &mut self.current_screen {
                            CurrentScreen::Welcome(screen) => {
                                screen.handle_event(&Event::Key(key_event))
                            }
                            CurrentScreen::Countdown(screen) => {
                                screen.handle_event(&Event::Key(key_event))
                            }
                        } {
                            self.handle_action(action);
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_action(&mut self, action: Action) {
        match action {
            Action::Quit => self.exit = true,
            Action::SetDurations {
                work_duration_minutes,
                break_duration_minutes,
            } => {
                self.work_duration_minutes = work_duration_minutes;
                self.break_duration_minutes = break_duration_minutes;

                self.current_screen = CurrentScreen::Countdown(CountdownScreen::new(
                    self.work_duration_minutes,
                    self.break_duration_minutes,
                ))
            }
        }
    }
}
