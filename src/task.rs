use std::fmt::Display;

use ratatui::{text::Line, widgets::ListItem};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
}
