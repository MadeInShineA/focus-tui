use ratatui::{Frame, crossterm::event::Event, layout::Rect};

use crate::{
    app::{Action, Popup},
    popups::task_list::TaskStatus,
};

pub struct AddTaskPopup {
    current_title: String,
    current_status: TaskStatus,
}

impl Popup for AddTaskPopup {
    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        todo!()
    }

    fn handle_event(&mut self, event: &Event) -> Option<Action> {
        todo!()
    }
}

