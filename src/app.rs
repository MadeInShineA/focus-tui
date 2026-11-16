use std::{io, time::Duration};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, poll},
    layout::Rect,
    widgets::{Block, Borders},
};
use uuid::Uuid;

use crate::{
    popups::{
        add_task::AddTaskPopup, edit_task::EditTaskPopup, error::ErrorPopup,
        task_list::TaskListPopup,
    },
    task::Task,
    task_manager::{SaveTaskError, TaskManager},
    theme::Theme,
};
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
    AddTask {
        task: Task,
    },
    DeleteTask {
        task_uuid: Uuid,
    },
    EditTask {
        return_to: usize,
        task: Task,
    },
    OpenTaskList {
        selected_task_idx: usize,
    },
    OpenAddTask {
        return_to: usize,
    },
    OpenEditTask {
        return_to: usize,
        selected_task: Task,
    },
    CycleTaskStatus {
        task_uuid: Uuid,
    },
    ShowError(String),
    ClosePopup,
}

pub trait Screen {
    fn draw(&self, frame: &mut Frame, area: Rect, theme: &Theme);
    fn handle_event(&mut self, event: &Event) -> Option<Action>;
    fn update(&mut self);
}

pub enum Popup {
    TaskList(TaskListPopup),
    AddTask(AddTaskPopup),
    EditTask(EditTaskPopup),
    Error(ErrorPopup),
}

impl Popup {
    pub fn draw(&mut self, frame: &mut Frame, area: Rect, theme: &Theme, tasks: &[Task]) {
        match self {
            Popup::TaskList(p) => p.draw(frame, area, theme, tasks),
            Popup::AddTask(p) => p.draw(frame, area, theme),
            Popup::EditTask(p) => p.draw(frame, area, theme),
            Popup::Error(p) => p.draw(frame, area, theme),
        }
    }

    pub fn handle_event(&mut self, event: &Event, tasks: &[Task]) -> Option<Action> {
        match self {
            Popup::TaskList(p) => p.handle_event(event, tasks),
            Popup::AddTask(p) => p.handle_event(event),
            Popup::EditTask(p) => p.handle_event(event),
            Popup::Error(p) => p.handle_event(event),
        }
    }
}

pub struct App {
    task_manager: TaskManager,
    current_screen: Box<dyn Screen>,
    popups: Vec<Popup>,
    theme: Theme,
    work_duration_minutes: u64,
    break_duration_minutes: u64,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            // TODO: Handle errors
            task_manager: TaskManager::new("./tasks.json").unwrap(),
            current_screen: Box::new(WelcomeScreen::new()),
            popups: Vec::new(),
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
        if let Some(current_popup) = self.popups.last_mut() {
            current_popup.draw(frame, inner_area, &self.theme, &self.task_manager.tasks);
        }
    }

    fn handle_event(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => self.handle_action(Action::Quit),
                    KeyCode::Char('t') if self.popups.is_empty() => {
                        self.handle_action(Action::OpenTaskList {
                            selected_task_idx: 0,
                        })
                    }
                    _ => {
                        if let Some(action) = {
                            if let Some(current_popup) = self.popups.last_mut() {
                                current_popup
                                    .handle_event(&Event::Key(key_event), &self.task_manager.tasks)
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
            Action::OpenTaskList { selected_task_idx } => self
                .popups
                .push(Popup::TaskList(TaskListPopup::new(selected_task_idx))),

            Action::OpenAddTask { return_to } => self
                .popups
                .push(Popup::AddTask(AddTaskPopup::new(return_to))),

            Action::AddTask { task } => {
                let add_result: Result<usize, SaveTaskError> = self.task_manager.add_task(task);

                match add_result {
                    Ok(idx) => self.handle_action(Action::OpenTaskList {
                        selected_task_idx: (idx),
                    }),
                    Err(error) => self.handle_action(Action::ShowError(error.to_string())),
                }
            }
            Action::DeleteTask { task_uuid } => match self.task_manager.delete_task(task_uuid) {
                Ok(()) => {}
                Err(error) => self.handle_action(Action::ShowError(error.to_string())),
            },
            Action::OpenEditTask {
                return_to,
                selected_task,
            } => self.popups.push(Popup::EditTask(EditTaskPopup::new(
                return_to,
                selected_task,
            ))),
            Action::EditTask { return_to, task } => {
                let edit_result = self
                    .task_manager
                    .edit_task(task.uuid, task.title, task.status);

                match edit_result {
                    Ok(()) => self.handle_action(Action::OpenTaskList {
                        selected_task_idx: (return_to),
                    }),
                    Err(error) => self.handle_action(Action::ShowError(error.to_string())),
                }
            }
            Action::CycleTaskStatus { task_uuid } => {
                match self.task_manager.cycle_task(task_uuid) {
                    Ok(()) => {}
                    Err(error) => self.handle_action(Action::ShowError(error.to_string())),
                }
            }

            Action::ShowError(error) => self.popups.push(Popup::Error(ErrorPopup::new(error))),

            Action::ClosePopup => {
                self.popups.pop();
            }
        }
    }
}
