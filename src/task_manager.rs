use std::{
    fs::{read_to_string, write},
    path::{Path, PathBuf},
};

use uuid::Uuid;

use crate::task::{Task, TaskStatus};

pub struct TaskManager {
    pub tasks: Vec<Task>,
    tasks_file_path: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum LoadTaskFileError {
    #[error("An I/O error occurred while reading the file at {}: {}", .0.display(), .1)]
    IoError(PathBuf, std::io::Error),

    #[error("JSON parsing error in task file at {}: {}", .0.display(), .1)]
    ParsingError(PathBuf, serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SaveTaskError {
    #[error("Failed to serialize tasks to JSON for file '{}': {}", .0.display(), .1)]
    JsonConversionError(PathBuf, serde_json::Error),
    #[error("{}", describe_write_error(.0, .1))]
    FileWriteError(PathBuf, std::io::Error),
}

fn describe_write_error(path: &Path, error: &std::io::Error) -> String {
    let path = path.display();
    match error.kind() {
        std::io::ErrorKind::PermissionDenied => {
            format!("Permission denied when writing to '{path}'. Check file permissions.")
        }
        std::io::ErrorKind::NotFound => {
            format!("Cannot write to '{path}': parent directory does not exist")
        }
        _ => format!("Failed to write tasks to file '{path}': {error}"),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EditTaskError {
    #[error("No task with id {0}")]
    NotFound(Uuid),
    #[error(transparent)]
    Save(#[from] SaveTaskError),
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
                self.tasks = serde_json::from_str(&content).map_err(|error| {
                    LoadTaskFileError::ParsingError(self.tasks_file_path.clone(), error)
                })?;
            }
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => {
                    self.tasks = Vec::new();
                }
                _ => {
                    return Err(LoadTaskFileError::IoError(
                        self.tasks_file_path.clone(),
                        error,
                    ));
                }
            },
        }
        Ok(())
    }

    fn save_tasks(&self, tasks: &[Task]) -> Result<(), SaveTaskError> {
        let tasks_json_string: String = serde_json::to_string_pretty(tasks).map_err(|error| {
            SaveTaskError::JsonConversionError(self.tasks_file_path.clone(), error)
        })?;

        write(&self.tasks_file_path, tasks_json_string)
            .map_err(|error| SaveTaskError::FileWriteError(self.tasks_file_path.clone(), error))
    }

    pub fn add_task(&mut self, task: Task) -> Result<usize, SaveTaskError> {
        let mut new_tasks: Vec<Task> = self.tasks.clone();
        new_tasks.push(task.clone());

        self.save_tasks(&new_tasks)?;

        self.tasks.push(task);
        let idx = self.tasks.len() - 1;
        Ok(idx)
    }

    pub fn delete_task(&mut self, task_uuid: Uuid) -> Result<(), SaveTaskError> {
        let mut new_tasks: Vec<Task> = self.tasks.clone();
        new_tasks.retain(|task_element| task_element.uuid != task_uuid);

        self.save_tasks(&new_tasks)?;

        self.tasks = new_tasks;
        Ok(())
    }

    pub fn edit_task(
        &mut self,
        task_uuid: Uuid,
        new_task_title: String,
        new_task_status: TaskStatus,
    ) -> Result<(), EditTaskError> {
        let mut new_tasks = self.tasks.clone();

        if let Some(edited_task) = new_tasks.iter_mut().find(|task| task.uuid == task_uuid) {
            edited_task.title = new_task_title;
            edited_task.status = new_task_status;

            self.save_tasks(&new_tasks)?;
            self.tasks = new_tasks;

            Ok(())
        } else {
            Err(EditTaskError::NotFound(task_uuid))
        }
    }

    pub fn cycle_task(&mut self, task_uuid: Uuid) -> Result<(), SaveTaskError> {
        let mut new_tasks = self.tasks.clone();
        if let Some(task) = new_tasks.iter_mut().find(|t| t.uuid == task_uuid) {
            task.cycle_status();
        }
        self.save_tasks(&new_tasks)?;
        self.tasks = new_tasks;
        Ok(())
    }
}
