use std::{io, time::Duration};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, poll},
    layout::Rect,
    widgets::{Block, Borders},
};

use crate::{popups::task_list::TaskListPopup, theme::Theme};
use crate::{
    screens::{countdown::CountdownScreen, welcome::WelcomeScreen},
    utils::{DEFAULT_BREAK_DURATION_MINUTES, DEFAULT_WORK_DURATION_MINUTES},
};

pub enum Action {
    Quit,
    SetDurations {
        work_duration_minutes: u64,
        break_duration_minutes: u64,
    },
}

pub trait Screen {
    fn draw(&self, frame: &mut Frame, area: Rect, theme: &Theme);
    fn handle_event(&mut self, event: &Event) -> Option<Action>;
    fn update(&mut self);
}

pub trait Popup {
    fn draw(&mut self, frame: &mut Frame, area: Rect, theme: &Theme);
    fn handle_event(&mut self, event: &Event) -> Option<Action>;
}

pub struct App {
    current_screen: Box<dyn Screen>,
    current_popup: Option<Box<dyn Popup>>,
    theme: Theme,
    work_duration_minutes: u64,
    break_duration_minutes: u64,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: Box::new(WelcomeScreen::new()),
            current_popup: None,
            theme: Theme::catppuccin_mocha(),
            work_duration_minutes: DEFAULT_WORK_DURATION_MINUTES,
            break_duration_minutes: DEFAULT_BREAK_DURATION_MINUTES,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.current_screen.update();
            if poll(Duration::from_millis(10))? {
                self.handle_event()?;
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(self.theme.border_style)
            .style(self.theme.background_style);
        frame.render_widget(block.clone(), frame.area());
        let inner_area = block.inner(frame.area());
        self.current_screen.draw(frame, inner_area, &self.theme);
        if let Some(current_popup) = &mut self.current_popup {
            current_popup.draw(frame, inner_area, &self.theme);
        }
    }

    fn handle_event(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => self.handle_action(Action::Quit),
                    KeyCode::Char('t') => match self.current_popup {
                        None => self.current_popup = Some(Box::new(TaskListPopup::new())),
                        _ => self.current_popup = None,
                    },
                    _ => {
                        if let Some(action) = {
                            if let Some(current_popup) = &mut self.current_popup {
                                current_popup.handle_event(&Event::Key(key_event))
                            } else {
                                self.current_screen.handle_event(&Event::Key(key_event))
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

                self.current_screen = Box::new(CountdownScreen::new(
                    self.work_duration_minutes,
                    self.break_duration_minutes,
                ));
            }
        }
    }
}
