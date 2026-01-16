use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
};

use crate::screens::{welcome::WelcomeScreen, work_countdown::WorkCountdownScreen};

pub enum Action {
    Quit,
    SwitchToWorkCountdownScreen,
}
pub trait Screen {
    fn draw(&self, frame: &mut Frame);
    fn handle_events(&mut self, event: &Event) -> Option<Action>;
    fn update(&mut self);
}

enum CurrentScreen {
    Welcome(WelcomeScreen),
    WorkCountdown(WorkCountdownScreen),
}

pub struct App {
    current_screen: CurrentScreen,
    // Initial time in seconds
    initial_time: i32,
    // Remaining time in seconds
    remaining_time: i32,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Welcome(WelcomeScreen),
            initial_time: 60,
            remaining_time: 60,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        match &self.current_screen {
            CurrentScreen::Welcome(screen) => screen.draw(frame),
            CurrentScreen::WorkCountdown(screen) => screen.draw(frame),
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => self.handle_action(Action::Quit),
                    _ => {
                        if let Some(action) = match &mut self.current_screen {
                            CurrentScreen::Welcome(screen) => {
                                screen.handle_events(&Event::Key(key_event))
                            }
                            CurrentScreen::WorkCountdown(screen) => {
                                screen.handle_events(&Event::Key(key_event))
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
            Action::SwitchToWorkCountdownScreen => {
                self.current_screen = CurrentScreen::WorkCountdown(WorkCountdownScreen {
                    initial_countdown_seconds: 60,
                    elapsed_time_seconds: 0,
                })
            }
        }
    }
}
