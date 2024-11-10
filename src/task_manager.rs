use crate::models::{Status, Task};
use chrono::Local;
use colored::*;
use serde_json;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub struct TaskManager {
    file_path: String,
    verbose: bool,
}

impl TaskManager {
    pub fn new(file_path: String, verbose: bool) -> Self {
        TaskManager { file_path, verbose }
    }

    pub fn load_tasks(&self) -> HashMap<u32, Task> {
        if !Path::new(&self.file_path).exists() {
            File::create(&self.file_path).expect("Failed to create JSON file.");
            return HashMap::new();
        }

        let mut file = File::open(&self.file_path).expect("Failed to open JSON file.");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Failed to read data.");

        if data.is_empty() {
            HashMap::new()
        } else {
            let tasks: Vec<Task> = serde_json::from_str(&data).expect("Failed to parse JSON.");
            tasks.into_iter().map(|task| (task.id, task)).collect()
        }
    }

    fn save_tasks(&self, tasks: &HashMap<u32, Task>) {
        let mut tasks_vec: Vec<&Task> = tasks.values().collect();
        tasks_vec.sort_by(|a, b| a.id.cmp(&b.id));

        let data = serde_json::to_string(&tasks_vec).expect("Failed to serialize tasks.");
        let mut file = File::create(&self.file_path).expect("Failed to create JSON file.");
        file.write_all(data.as_bytes())
            .expect("Failed to write data.");
    }

    pub fn add_task(&self, description: String) {
        let mut tasks = self.load_tasks();
        let id = tasks.keys().max().cloned().unwrap_or(0) + 1;

        let task = Task {
            id,
            description,
            status: Status::Todo,
            created_at: Local::now(),
            updated_at: Local::now(),
        };

        tasks.insert(id, task);
        if self.verbose {
            println!(
                "{}",
                format!("Task added successfully with ID: {}", id).green()
            );
        }

        self.save_tasks(&tasks);
        self.list_tasks(None);
    }

    pub fn update_task(&self, id: u32, description: String) {
        let mut tasks = self.load_tasks();
        match tasks.get_mut(&id) {
            Some(task) => {
                task.description = description;
                task.updated_at = Local::now();
                if self.verbose {
                    println!(
                        "{}",
                        format!("Task with ID {} updated successfully.", id).green()
                    );
                }

                self.save_tasks(&tasks);
                self.list_tasks(None);
            }
            None => {
                if self.verbose {
                    eprintln!("{}", format!("Task with ID {} not found.", id).red());
                }
            }
        }
    }

    pub fn delete_task(&self, id: u32) {
        let mut tasks = self.load_tasks();
        if tasks.remove(&id).is_some() {
            if self.verbose {
                println!(
                    "{}",
                    format!("Task with ID {} deleted successfully.", id).green()
                );
            }

            self.save_tasks(&tasks);
            self.list_tasks(None);
        } else {
            if self.verbose {
                eprintln!("{}", format!("Task with ID {} not found.", id).red());
            }
        }
    }

    pub fn mark_in_progress_task(&self, id: u32) {
        let mut tasks = self.load_tasks();
        match tasks.get_mut(&id) {
            Some(task) => {
                if task.status == Status::InProgress {
                    if self.verbose {
                        eprintln!(
                            "{}",
                            format!("Task with ID {} is already in progress.", id).yellow()
                        );
                    }
                } else {
                    task.status = Status::InProgress;
                    task.updated_at = Local::now();
                    if self.verbose {
                        println!(
                            "{}",
                            format!("Task with ID {} marked as in progress.", id).green()
                        );
                    }

                    self.save_tasks(&tasks);
                }
                self.list_tasks(None);
            }
            None => {
                if self.verbose {
                    eprintln!("{}", format!("Task with ID {} not found.", id).red());
                }
            }
        }
    }

    pub fn mark_done(&self, id: u32) {
        let mut tasks = self.load_tasks();
        match tasks.get_mut(&id) {
            Some(task) => {
                if task.status == Status::Done {
                    if self.verbose {
                        eprintln!(
                            "{}",
                            format!("Task with ID {} is already done.", id).yellow()
                        );
                    }
                } else {
                    task.status = Status::Done;
                    task.updated_at = Local::now();
                    if self.verbose {
                        println!("{}", format!("Task with ID {} marked as done.", id).green());
                    }

                    self.save_tasks(&tasks);
                }

                self.list_tasks(None);
            }
            None => {
                if self.verbose {
                    eprintln!("{}", format!("Task with ID {} not found.", id).red());
                }
            }
        }
    }

    pub fn restart_task(&self, id: u32) {
        let mut tasks = self.load_tasks();
        match tasks.get_mut(&id) {
            Some(task) => {
                if task.status == Status::Todo {
                    if self.verbose {
                        eprintln!(
                            "{}",
                            format!("Task with ID {} is already in Todo status.", id).yellow()
                        );
                    }
                } else {
                    task.status = Status::Todo;
                    task.updated_at = Local::now();
                    if self.verbose {
                        println!(
                            "{}",
                            format!("Task with ID {} restarted successfully.", id).green()
                        );
                    }

                    self.save_tasks(&tasks);
                }
                self.list_tasks(None);
            }
            None => {
                if self.verbose {
                    eprintln!("{}", format!("Task with ID {} not found.", id).red());
                }
            }
        }
    }

    pub fn list_tasks(&self, filter: Option<String>) -> Vec<Task> {
        let tasks = self.load_tasks();

        let mut filtered_tasks: Vec<Task> = match filter.as_deref() {
            Some("all") | None => tasks.values().cloned().collect(),
            Some("done") => tasks
                .values()
                .filter(|task| task.status == Status::Done)
                .cloned()
                .collect(),
            Some("in-progress") => tasks
                .values()
                .filter(|task| task.status == Status::InProgress)
                .cloned()
                .collect(),
            Some("not-done") => tasks
                .values()
                .filter(|task| task.status != Status::Done)
                .cloned()
                .collect(),
            _ => {
                if self.verbose {
                    eprintln!("{}", "Usage: task-tracker-cli list [all | done | in-progress | not-done], default is all.".red());
                }
                return Vec::new();
            }
        };

        if filtered_tasks.is_empty() {
            if self.verbose {
                println!("{}", "No tasks found.".yellow());
            }
            return Vec::new();
        }

        filtered_tasks.sort_by_key(|task| task.id);

        if self.verbose {
            println!(
                "{: <5} | {: <40} | {: <15} | {: <20} | {: <20}",
                "ID".bold(),
                "Description".bold(),
                "Status".bold(),
                "Created At".bold(),
                "Updated At".bold()
            );
            println!("{}", "-".repeat(111).bright_black());

            for task in &filtered_tasks {
                let status_colored = match task.status {
                    Status::Todo => "Todo".yellow(),
                    Status::InProgress => "InProgress".blue(),
                    Status::Done => "Done".green(),
                };
                println!(
                    "{: <5} | {: <40} | {: <15} | {: <20} | {: <20}",
                    task.id,
                    task.description,
                    status_colored,
                    task.created_at.format("%d-%m-%Y %H:%M:%S"),
                    task.updated_at.format("%d-%m-%Y %H:%M:%S")
                );
            }
        }

        filtered_tasks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_task_manager() -> TaskManager {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let file_path = temp_file.path().to_str().unwrap().to_string();
        TaskManager::new(file_path, false)
    }

    #[test]
    fn test_load_tasks_with_empty_file() {
        let task_manager = create_task_manager();
        let tasks = task_manager.load_tasks();

        assert!(tasks.is_empty(), "Expected no tasks in empty file.");
    }

    #[test]
    fn test_add_task() {
        let task_manager = create_task_manager();
        task_manager.add_task("Test task".to_string());
        let tasks = task_manager.load_tasks();

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[&1].description, "Test task");
        assert_eq!(tasks[&1].status, Status::Todo);
    }

    #[test]
    fn test_update_task() {
        let task_manager = create_task_manager();
        task_manager.add_task("Test task".to_string());

        task_manager.update_task(1, "Updated task".to_string());
        let tasks = task_manager.load_tasks();

        assert_eq!(tasks[&1].description, "Updated task");
    }

    #[test]
    fn test_delete_task() {
        let task_manager = create_task_manager();
        task_manager.add_task("Test task".to_string());

        task_manager.delete_task(1);
        let tasks = task_manager.load_tasks();

        assert!(tasks.is_empty());
    }

    #[test]
    fn test_delete_task_not_found() {
        let task_manager = create_task_manager();
        task_manager.add_task("Test task".to_string());

        task_manager.delete_task(2);
        let tasks = task_manager.load_tasks();

        assert_eq!(tasks.len(), 1);
    }

    #[test]
    fn test_mark_in_progress_task() {
        let task_manager = create_task_manager();
        task_manager.add_task("Test task".to_string());

        task_manager.mark_in_progress_task(1);
        let tasks = task_manager.load_tasks();

        assert_eq!(tasks[&1].status, Status::InProgress);
    }

    #[test]
    fn test_mark_done() {
        let task_manager = create_task_manager();
        task_manager.add_task("Test task".to_string());

        task_manager.mark_done(1);
        let tasks = task_manager.load_tasks();

        assert_eq!(tasks[&1].status, Status::Done);
    }

    #[test]
    fn test_restart_task() {
        let task_manager = create_task_manager();
        task_manager.add_task("Test task".to_string());
        task_manager.mark_done(1);

        task_manager.restart_task(1);
        let tasks = task_manager.load_tasks();

        assert_eq!(tasks[&1].status, Status::Todo);
    }

    #[test]
    fn test_list_tasks() {
        let task_manager = create_task_manager();
        task_manager.add_task("Task 1".to_string());
        task_manager.add_task("Task 2".to_string());
        task_manager.mark_done(2);

        let tasks = task_manager.list_tasks(Some("done".to_string()));

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, "Task 2");
        assert_eq!(tasks[0].status, Status::Done);
    }
}
