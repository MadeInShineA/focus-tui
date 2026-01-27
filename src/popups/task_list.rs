use std::fmt::Display;

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Text},
    widgets::{Block, Clear, List, ListItem, ListState, Paragraph},
};
use serde::{Deserialize, Serialize};

use crate::{
    app::{Action, Popup},
    popups::add_task::AddTaskPopup,
    theme::Theme,
    utils::popup_area,
};

#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
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
    tasks: Vec<Task>,
    list_state: ListState,
}

impl TaskListPopup {
    pub fn new() -> Self {
        TaskListPopup {
            tasks: Vec::new(),
            list_state: ListState::default(),
        }
    }

    pub fn from_iter(tasks: &[Task]) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        TaskListPopup {
            tasks: tasks.to_vec(),
            list_state,
        }
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) -> Option<Action> {
        match key_event.code {
            KeyCode::Char('t') => return Some(Action::ClosePopup),
            KeyCode::Char('a') => {
                return Some(Action::OpenPopup {
                    popup: Box::new(AddTaskPopup::new()),
                });
            }
            KeyCode::Up => {
                if let Some(selected_index) = &self.list_state.selected() {
                    let new_selected_index: usize = selected_index.saturating_sub(1);
                    self.list_state.select(Some(new_selected_index));
                }
            }
            KeyCode::Down => {
                if let Some(selected_index) = &self.list_state.selected() {
                    let new_selected_index =
                        usize::min(selected_index.saturating_add(1), self.tasks.len() - 1);
                    self.list_state.select(Some(new_selected_index));
                }
            }
            _ => {}
        }
        None
    }

    fn add_task(&mut self, title: String, status: TaskStatus) {
        let task: Task = { Task { title, status } };
        self.tasks.push(task);
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
            self.tasks
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
