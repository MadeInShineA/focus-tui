use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

use uuid::Uuid;

use crate::popups::task_list::{Task, TaskStatus};

pub struct TaskManager {
    pub tasks: Vec<Task>,
    tasks_file_path: PathBuf,
}

#[derive(Debug)]
pub enum LoadTaskFileError {
    NotFound,
    IoError(std::io::Error),
    ParsingError(serde_json::Error),
}

#[derive(Debug)]
pub enum SaveTaskError {
    TasksJsonConversionError(serde_json::Error),
    FileWriteError(std::io::Error),
}

impl TaskManager {
    pub fn new(file_path: &str) -> Result<Self, LoadTaskFileError> {
        let mut manager = TaskManager {
            tasks: Vec::new(),
            tasks_file_path: PathBuf::from(file_path),
        };

        manager.load_tasks()?;
        Ok(manager)
    }

    fn load_tasks(&mut self) -> Result<(), LoadTaskFileError> {
        match read_to_string(&self.tasks_file_path) {
            Ok(content) => {
                self.tasks =
                    serde_json::from_str(&content).map_err(LoadTaskFileError::ParsingError)?;
            }
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => {
                    self.tasks = Vec::new();
                }
                _ => {
                    return Err(LoadTaskFileError::IoError(error));
                }
            },
        }
        Ok(())
    }

    fn save_tasks(&self) -> Result<(), SaveTaskError> {
        let tasks_json_string: String = serde_json::to_string_pretty(&self.tasks)
            .map_err(SaveTaskError::TasksJsonConversionError)?;

        write(&self.tasks_file_path, tasks_json_string).map_err(SaveTaskError::FileWriteError)
    }

    pub fn add_task(&mut self, task: Task) -> Result<(), SaveTaskError> {
        self.tasks.push(task);
        self.save_tasks()
    }

    pub fn delete_task(&mut self, task_uuid: Uuid) -> Result<(), SaveTaskError> {
        self.tasks
            .retain(|task_element| task_element.uuid != task_uuid);
        self.save_tasks()
    }

    pub fn edit_task(
        &mut self,
        task_uuid: Uuid,
        new_task_title: String,
        new_task_status: TaskStatus,
    ) {
        for task in self.tasks.iter_mut() {
            if task.uuid == task_uuid {
                task.title = new_task_title;
                task.status = new_task_status;
                break;
            }
        }
    }
}
