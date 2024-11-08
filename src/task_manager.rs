use crate::models::{Status, Task};
use chrono::Local;
use colored::*;
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
    pub fn load_tasks() -> HashMap<u32, Task> {
        if !Path::new(FILE_PATH).exists() {
            File::create(FILE_PATH).expect("Failed to create JSON file.");
            return HashMap::new();
        }

        let mut file = File::open(FILE_PATH).expect("Failed to open JSON file.");
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
        println!(
            "{}",
            format!("Task added successfully with ID: {}", id).green()
        );

        Self::save_tasks(&tasks);
        Self::list_tasks(None);
    }

    pub fn update_task(id: u32, description: String) {
        let mut tasks = Self::load_tasks();
        match tasks.get_mut(&id) {
            Some(task) => {
                task.description = description;
                task.updated_at = Local::now();
                println!(
                    "{}",
                    format!("Task with ID {} updated successfully.", id).green()
                );

                Self::save_tasks(&tasks);
                Self::list_tasks(None);
            }
            None => {
                eprintln!("{}", format!("Task with ID {} not found.", id).red());
            }
        }
    }

    pub fn delete_task(id: u32) {
        let mut tasks = Self::load_tasks();
        if tasks.remove(&id).is_some() {
            println!(
                "{}",
                format!("Task with ID {} deleted successfully.", id).green()
            );

            Self::save_tasks(&tasks);
            Self::list_tasks(None);
        } else {
            eprintln!("{}", format!("Task with ID {} not found.", id).red());
        }
    }

    pub fn mark_in_progress_task(id: u32) {
        let mut tasks = Self::load_tasks();
        match tasks.get_mut(&id) {
            Some(task) => {
                if task.status == Status::InProgress {
                    eprintln!(
                        "{}",
                        format!("Task with ID {} is already in progress.", id).yellow()
                    )
                } else {
                    task.status = Status::InProgress;
                    task.updated_at = Local::now();
                    println!(
                        "{}",
                        format!("Task with ID {} marked as in progress.", id).green()
                    );

                    Self::save_tasks(&tasks);
                }
                Self::list_tasks(None);
            }
            None => {
                eprintln!("{}", format!("Task with ID {} not found.", id).red());
            }
        }
    }

    pub fn mark_done(id: u32) {
        let mut tasks = Self::load_tasks();
        match tasks.get_mut(&id) {
            Some(task) => {
                if task.status == Status::Done {
                    eprintln!(
                        "{}",
                        format!("Task with ID {} is already done.", id).yellow()
                    );
                } else {
                    task.status = Status::Done;
                    task.updated_at = Local::now();
                    println!("{}", format!("Task with ID {} marked as done.", id).green());

                    Self::save_tasks(&tasks);
                }

                Self::list_tasks(None);
            }
            None => {
                eprintln!("{}", format!("Task with ID {} not found.", id).red());
            }
        }
    }

    pub fn restart_task(id: u32) {
        let mut tasks = Self::load_tasks();
        match tasks.get_mut(&id) {
            Some(task) => {
                if task.status == Status::Todo {
                    eprintln!(
                        "{}",
                        format!("Task with ID {} is already in Todo status.", id).yellow()
                    );
                } else {
                    task.status = Status::Todo;
                    task.updated_at = Local::now();
                    println!(
                        "{}",
                        format!("Task with ID {} restarted successfully.", id).green()
                    );

                    Self::save_tasks(&tasks);
                }
                Self::list_tasks(None);
            }
            None => {
                eprintln!("{}", format!("Task with ID {} not found.", id).red());
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
                eprintln!("{}", "Usage: task-tracker-cli list [all | done | in-progress | not-done], default is all.".red());
                return;
            }
        };

        if filtered_tasks.is_empty() {
            println!("{}", "No tasks found.".yellow());
            return;
        }

        filtered_tasks.sort_by_key(|task| task.id);

        println!(
            "{: <5} | {: <40} | {: <15} | {: <20} | {: <20}",
            "ID".bold(),
            "Description".bold(),
            "Status".bold(),
            "Created At".bold(),
            "Updated At".bold()
        );
        println!("{}", "-".repeat(111).bright_black());

        for task in filtered_tasks {
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
}
