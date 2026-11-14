use std::fmt::Display;

use ratatui::{text::Line, widgets::ListItem};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Todo,
    Ongoing,
    Done,
}

impl TaskStatus {
    pub fn next(&self) -> TaskStatus {
        match self {
            TaskStatus::Todo => TaskStatus::Ongoing,
            TaskStatus::Ongoing => TaskStatus::Done,
            TaskStatus::Done => TaskStatus::Todo,
        }
    }

    pub fn previous(&self) -> TaskStatus {
        match self {
            TaskStatus::Todo => TaskStatus::Done,
            TaskStatus::Ongoing => TaskStatus::Todo,
            TaskStatus::Done => TaskStatus::Ongoing,
        }
    }

    pub fn emoji(&self) -> String {
        match self {
            TaskStatus::Todo => String::from('❌'),
            TaskStatus::Ongoing => String::from('⏳'),
            TaskStatus::Done => String::from('✅'),
        }
    }
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "Todo"),
            TaskStatus::Ongoing => write!(f, "Ongoing"),
            TaskStatus::Done => write!(f, "Done"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub uuid: Uuid,
    pub title: String,
    pub status: TaskStatus,
}

impl Task {
    pub fn get_list_item<'a>(&self) -> ListItem<'a> {
        let content: String = format!("{} {}", self.status.emoji(), self.title);
        let line: Line = Line::from(content).centered();
        ListItem::new(line)
    }

    pub fn cycle_status(&mut self) {
        self.status = self.status.next();
    }
}
