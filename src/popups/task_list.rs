use std::{cell::RefCell, fmt::Display, rc::Rc};

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Text},
    widgets::{Block, Clear, List, ListItem, ListState, Paragraph},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app::{Action, Popup},
    popups::add_task::AddTaskPopup,
    storage::TaskManager,
    theme::Theme,
    utils::popup_area,
};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Done,
    Ongoing,
    Todo,
}

impl TaskStatus {
    pub fn next(&self) -> TaskStatus {
        match self {
            TaskStatus::Done => TaskStatus::Ongoing,
            TaskStatus::Ongoing => TaskStatus::Todo,
            TaskStatus::Todo => TaskStatus::Done,
        }
    }

    pub fn previous(&self) -> TaskStatus {
        match self {
            TaskStatus::Done => TaskStatus::Todo,
            TaskStatus::Ongoing => TaskStatus::Done,
            TaskStatus::Todo => TaskStatus::Ongoing,
        }
    }

    pub fn emoji(&self) -> String {
        match self {
            TaskStatus::Done => String::from('✅'),
            TaskStatus::Ongoing => String::from('⏳'),
            TaskStatus::Todo => String::from('❌'),
        }
    }
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Done => write!(f, "Done"),
            TaskStatus::Ongoing => write!(f, "Ongoing"),
            TaskStatus::Todo => write!(f, "Todo"),
        }
    }
}

// TODO: use an id / uuid ?
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub uuid: Uuid,
    pub title: String,
    pub status: TaskStatus,
}

impl Task {
    fn get_list_item<'a>(&self) -> ListItem<'a> {
        let content: String = format!("{} {}", self.status.emoji(), self.title);
        let line: Line = Line::from(content).centered();
        ListItem::new(line)
    }
}

pub struct TaskListPopup {
    task_manager: Rc<RefCell<TaskManager>>,
    list_state: ListState,
}

impl TaskListPopup {
    pub fn new(task_manager: Rc<RefCell<TaskManager>>, selected_task_idx: usize) -> Self {
        let mut list_state = ListState::default();
        let task_count = task_manager.borrow().tasks.len();
        if task_count > 0 {
            list_state.select(Some(selected_task_idx)); // Select first item by default
        }

        TaskListPopup {
            task_manager,
            list_state: list_state,
        }
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) -> Option<Action> {
        match key_event.code {
            KeyCode::Char('t') | KeyCode::Esc => return Some(Action::ClosePopup),
            KeyCode::Char('a') => {
                return Some(Action::OpenPopup {
                    popup: Box::new(AddTaskPopup::new()),
                });
            }
            KeyCode::Char('d') => {
                if let Some(selected_index) = &self.list_state.selected() {
                    let task_manager_borrowed = self.task_manager.borrow();
                    let selected_task_uuid = task_manager_borrowed.tasks[*selected_index].uuid;
                    drop(task_manager_borrowed);

                    let _ = self
                        .task_manager
                        .borrow_mut()
                        .delete_task(selected_task_uuid);

                    let new_len = self.task_manager.borrow().tasks.len();
                    if new_len == 0 {
                        self.list_state.select(None);
                    } else if *selected_index >= new_len {
                        self.list_state.select(Some(new_len - 1));
                    }
                }
            }
            KeyCode::Up => {
                if let Some(selected_index) = &self.list_state.selected() {
                    let new_selected_index: usize = selected_index.saturating_sub(1);
                    self.list_state.select(Some(new_selected_index));
                }
            }
            KeyCode::Down => {
                if let Some(selected_index) = &self.list_state.selected() {
                    let new_selected_index = usize::min(
                        selected_index.saturating_add(1),
                        self.task_manager.borrow().tasks.len() - 1,
                    );
                    self.list_state.select(Some(new_selected_index));
                }
            }
            _ => {}
        }
        None
    }
}

impl Popup for TaskListPopup {
    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect, theme: &Theme) {
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

        let title_text: Text = Text::styled("Task list popup", theme.text_style()).centered();
        let title_paragraphe: Paragraph = Paragraph::new(title_text).centered();

        let task_list: List = List::new(
            self.task_manager
                .borrow()
                .tasks
                .iter()
                .map(|task| task.get_list_item().style(theme.text_style()))
                .collect::<Vec<ListItem>>(),
        )
        .highlight_style(Style::default().fg(theme.text_color).bold());

        frame.render_widget(block, popup_area);
        frame.render_widget(title_paragraphe, inner_layout[0]);
        frame.render_stateful_widget(task_list, inner_layout[1], &mut self.list_state);
    }

    fn handle_event(&mut self, event: &Event) -> Option<Action> {
        match event {
            Event::Key(key_event) => self.handle_key_event(key_event),

            _ => None,
        }
    }
}
