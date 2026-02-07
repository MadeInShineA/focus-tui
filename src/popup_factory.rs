use std::{cell::RefCell, rc::Rc};

use crate::{
    app::Popup,
    popups::{add_task::AddTaskPopup, task_list::TaskListPopup},
    storage::TaskManager,
};

#[derive(Clone)]
pub struct PopupFactory {
    pub task_manager: Rc<RefCell<TaskManager>>,
}

impl PopupFactory {
    pub fn new(task_manager: Rc<RefCell<TaskManager>>) -> Self {
        PopupFactory { task_manager }
    }

    pub fn create_task_list_popup(&self, selected_task_idx: usize) -> Box<dyn Popup> {
        Box::new(TaskListPopup::new(
            Rc::new(self.clone()),
            self.task_manager.clone(),
            selected_task_idx,
        ))
    }

    pub fn create_add_task_popup(&self, task_opened_on_idx: usize) -> Box<dyn Popup> {
        Box::new(AddTaskPopup::new(Rc::new(self.clone()), task_opened_on_idx))
    }
}
