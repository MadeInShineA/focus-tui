use std::{
    fmt::Display,
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
    IoError(PathBuf, std::io::Error),
    ParsingError(PathBuf, serde_json::Error),
}

impl Display for LoadTaskFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadTaskFileError::IoError(path, error) => write!(
                f,
                "An I/O error occurred while reading the file at {}: {}",
                path.display(),
                error
            ),
            LoadTaskFileError::ParsingError(path, error) => write!(
                f,
                "JSON parsing error in task file at {}: {}",
                path.display(),
                error
            ),
        }
    }
}

pub enum SaveTaskError {
    JsonConversionError(PathBuf, serde_json::Error),
    FileWriteError(PathBuf, std::io::Error),
}

impl Display for SaveTaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveTaskError::JsonConversionError(path, error) => {
                write!(
                    f,
                    "Failed to serialize tasks to JSON for file '{}': {}",
                    path.display(),
                    error
                )
            }
            SaveTaskError::FileWriteError(path, error) => {
                if error.kind() == std::io::ErrorKind::PermissionDenied {
                    write!(
                        f,
                        "Permission denied when writing to '{}'. Check file permissions.",
                        path.display()
                    )
                } else if error.kind() == std::io::ErrorKind::NotFound {
                    write!(
                        f,
                        "Cannot write to '{}': parent directory does not exist",
                        path.display()
                    )
                } else {
                    write!(
                        f,
                        "Failed to write tasks to file '{}': {}",
                        path.display(),
                        error
                    )
                }
            }
        }
    }
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

        self.tasks
            .retain(|task_element| task_element.uuid != task_uuid);
        Ok(())
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
