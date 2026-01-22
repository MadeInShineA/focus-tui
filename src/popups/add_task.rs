use ratatui::{Frame, crossterm::event::Event, layout::Rect};

use crate::{
    app::{Action, Popup},
    popups::task_list::TaskStatus,
    theme::Theme,
};

pub struct AddTaskPopup {
    current_title: String,
    current_status: TaskStatus,
}

impl Popup for AddTaskPopup {
    fn draw(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        todo!()
    }

    fn handle_event(&mut self, event: &Event) -> Option<Action> {
        todo!()
    }
}
