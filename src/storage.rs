use std::{fs::read_to_string, path::PathBuf};

use crate::popups::task_list::Task;

pub struct TaskManager {
    tasks: Vec<Task>,
    tasks_file_path: PathBuf,
}

#[derive(Debug)]
pub enum LoadTaskFileError {
    NotFound,
    IoError(std::io::Error),
    ParsingError(serde_json::Error),
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
        let file_content = read_to_string(&self.tasks_file_path).map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                LoadTaskFileError::NotFound
            } else {
                LoadTaskFileError::IoError(error)
            }
        })?;

        self.tasks =
            serde_json::from_str(&file_content).map_err(LoadTaskFileError::ParsingError)?;

        Ok(())
    }
}
