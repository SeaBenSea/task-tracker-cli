use crate::models::{Status, Task};
use chrono::Local;
use serde_json::json;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
};

const FILE_PATH: &str = "tasks.json";

pub struct TaskManager;

impl TaskManager {
    fn load_tasks() -> HashMap<u32, Task> {
        if !Path::new(FILE_PATH).exists() {
            File::create(FILE_PATH).expect("Failed to create JSON file.");
            return HashMap::new();
        }

        let mut file = File::open(FILE_PATH).expect("Failed to open JSON file.");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Failed to read data.");

        if data.is_empty() {
            return HashMap::new();
        } else {
            let tasks: Vec<Task> = serde_json::from_str(&data).expect("Failed to parse JSON.");
            let tasks_map = tasks.into_iter().map(|task| (task.id, task)).collect();
            return tasks_map;
        }
    }

    fn save_tasks(tasks: &HashMap<u32, Task>) {
        let mut tasks_vec: Vec<&Task> = tasks.values().collect();
        tasks_vec.sort_by(|a, b| a.id.cmp(&b.id));

        let data = json!(tasks_vec);
        let mut file = File::create(FILE_PATH).expect("Failed to create JSON file.");
        file.write_all(data.to_string().as_bytes())
            .expect("Failed to write data.");
    }

    pub fn add_task(description: String) {
        let mut tasks = Self::load_tasks();
        let id = tasks.keys().max().cloned().unwrap_or(0) + 1;

        let task = Task {
            id,
            description,
            status: Status::Todo,
            created_at: Local::now(),
            updated_at: Local::now(),
        };

        tasks.insert(id, task);
        println!("Task added successfully with ID: {}", id);

        Self::save_tasks(&tasks);
        Self::list_tasks(None);
    }

    pub fn update_task(id: u32, description: String) {
        let mut tasks = Self::load_tasks();
        match tasks.get_mut(&id) {
            Some(task) => {
                task.description = description;
                task.updated_at = Local::now();
                println!("Task with ID {} updated successfully.", id);

                Self::save_tasks(&tasks);
                Self::list_tasks(None);
            }
            None => {
                eprintln!("Task with ID {} not found.", id);
            }
        }
    }

    pub fn delete_task(id: u32) {
        let mut tasks = Self::load_tasks();
        if tasks.remove(&id).is_some() {
            println!("Task with ID {} deleted successfully.", id);

            Self::save_tasks(&tasks);
            Self::list_tasks(None);
        } else {
            eprintln!("Task with ID {} not found.", id);
        }
    }

    pub fn mark_in_progress_task(id: u32) {
        let mut tasks = Self::load_tasks();
        match tasks.get_mut(&id) {
            Some(task) => {
                if task.status == Status::InProgress {
                    eprintln!("Task with ID {} is already in progress.", id);
                } else {
                    task.status = Status::InProgress;
                    task.updated_at = Local::now();
                    println!("Task with ID {} marked as in progress.", id);

                    Self::save_tasks(&tasks);
                }
                Self::list_tasks(None);
            }
            None => {
                eprintln!("Task with ID {} not found.", id);
            }
        }
    }

    pub fn mark_done(id: u32) {
        let mut tasks = Self::load_tasks();
        match tasks.get_mut(&id) {
            Some(task) => {
                if task.status == Status::Done {
                    eprintln!("Task with ID {} is already done.", id);
                } else {
                    task.status = Status::Done;
                    task.updated_at = Local::now();
                    println!("Task with ID {} marked as done.", id);

                    Self::save_tasks(&tasks);
                }

                Self::list_tasks(None);
            }
            None => {
                eprintln!("Task with ID {} not found.", id);
            }
        }
    }

    pub fn restart_task(id: u32) {
        let mut tasks = Self::load_tasks();
        match tasks.get_mut(&id) {
            Some(task) => {
                if task.status == Status::Todo {
                    eprintln!("Task with ID {} is already in Todo status.", id);
                } else {
                    task.status = Status::Todo;
                    task.updated_at = Local::now();
                    println!("Task with ID {} restarted successfully.", id);

                    Self::save_tasks(&tasks);
                }
                Self::list_tasks(None);
            }
            None => {
                eprintln!("Task with ID {} not found.", id);
            }
        }
    }

    pub fn list_tasks(filter: Option<String>) {
        let tasks = Self::load_tasks();

        let mut filtered_tasks: Vec<&Task> = match filter.as_deref() {
            Some("all") | None => tasks.values().collect(),
            Some("done") => tasks
                .values()
                .filter(|task| task.status == Status::Done)
                .collect(),
            Some("in-progress") => tasks
                .values()
                .filter(|task| task.status == Status::InProgress)
                .collect(),
            Some("not-done") => tasks
                .values()
                .filter(|task| task.status != Status::Done)
                .collect(),
            _ => {
                eprintln!("Usage: task-tracker-cli list [all | done | in-progress | not-done], default is all.");
                return;
            }
        };

        if filtered_tasks.is_empty() {
            println!("No tasks found.");
            return;
        }

        filtered_tasks.sort_by_key(|task| task.id);

        println!(
            "{: <5} | {: <20} | {: <10} | {: <20} | {: <20}",
            "ID", "Description", "Status", "Created At", "Updated At"
        );
        println!(
            "{:-<5} | {:-<20} | {:-<10} | {:-<20} | {:-<20}",
            "", "", "", "", ""
        );
        for task in filtered_tasks {
            println!(
                "{: <5} | {: <20} | {: <10} | {: <20} | {: <20}",
                task.id,
                task.description,
                format!("{:?}", task.status),
                task.created_at.format("%d-%m-%Y %H:%M:%S"),
                task.updated_at.format("%d-%m-%Y %H:%M:%S")
            );
        }
    }
}
