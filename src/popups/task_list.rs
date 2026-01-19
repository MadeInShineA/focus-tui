use std::fmt::Display;

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::Text,
    widgets::{Block, List, ListItem, ListState, Paragraph},
};

use crate::{
    app::{Action, Popup},
    utils::popup_area,
};

#[derive(Clone)]
pub enum TaskStatus {
    Done,
    Ongoing,
    Todo,
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

#[derive(Clone)]
pub struct Task {
    title: String,
    status: TaskStatus,
}

impl Task {
    fn get_list_item<'a>(&self) -> ListItem<'a> {
        ListItem::new(format!("{} {}", self.title, self.status))
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

    fn handle_key_event(&mut self, key_event: &KeyEvent) {
        match key_event.code {
            KeyCode::Char('a') => {
                self.add_task(String::from("Test"), TaskStatus::Done);
                let selected_index: usize = self.tasks.len() - 1;
                self.list_state.select(Some(selected_index));
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
    }

    fn add_task(&mut self, title: String, status: TaskStatus) {
        let task: Task = { Task { title, status } };
        self.tasks.push(task);
    }
}

impl Popup for TaskListPopup {
    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let block = Block::bordered();
        let popup_area: Rect = popup_area(area, 60, 60);
        let inner_area: Rect = block.inner(popup_area);
        let inner_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![Constraint::Length(1), Constraint::Fill(1)])
            .split(inner_area);

        let title_text: Text = Text::styled("Task list popup", Style::default()).centered();
        let title_paragraphe: Paragraph = Paragraph::new(title_text).centered();

        let task_list: List = List::new(
            self.tasks
                .iter()
                .map(|task| task.get_list_item())
                .collect::<Vec<ListItem>>(),
        )
        .highlight_style(Style::default().bold())
        .highlight_symbol(">>");

        frame.render_widget(block, popup_area);
        frame.render_widget(title_paragraphe, inner_layout[0]);
        frame.render_stateful_widget(task_list, inner_layout[1], &mut self.list_state);
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
}
