use crate::models::{Status, Task};
use chrono::Local;
use serde_json::json;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

const FILE_PATH: &str = "tasks.json";

pub struct TaskManager;

impl TaskManager {
    fn load_tasks() -> Vec<Task> {
        if !Path::new(FILE_PATH).exists() {
            File::create(FILE_PATH).expect("Failed to create JSON file.");
            return Vec::new();
        }

        let mut file = File::open(FILE_PATH).expect("Failed to open JSON file.");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Failed to read data.");

        if data.is_empty() {
            return Vec::new();
        } else {
            return serde_json::from_str(&data).expect("Failed to parse JSON.");
        }
    }

    fn save_tasks(tasks: &[Task]) {
        let mut tasks = tasks.to_vec();
        tasks.sort_by(|a, b| a.id.cmp(&b.id));

        let data = json!(tasks);
        let mut file = File::create(FILE_PATH).expect("Failed to create JSON file.");
        file.write_all(data.to_string().as_bytes())
            .expect("Failed to write data.");
    }

    pub fn add_task(description: String) {
        let mut tasks = Self::load_tasks();
        let id = if tasks.is_empty() {
            1
        } else {
            tasks.last().unwrap().id + 1
        };

        let task = Task {
            id,
            description,
            status: Status::Todo,
            created_at: Local::now(),
            updated_at: Local::now(),
        };

        tasks.push(task);
        print!("Task added successfully with ID: {}", id);

        Self::save_tasks(&tasks);
        Self::list_tasks(None);
    }

    pub fn update_task(id: u32, description: String) {
        let mut tasks = Self::load_tasks();
        let task = tasks.iter_mut().find(|task| task.id == id);

        match task {
            Some(task) => {
                task.description = description;
                task.updated_at = Local::now();

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
        let task = tasks.iter().position(|task| task.id == id);

        match task {
            Some(index) => {
                tasks.remove(index);
                Self::save_tasks(&tasks);
                Self::list_tasks(None);
            }
            None => {
                eprintln!("Task with ID {} not found.", id);
            }
        }
    }

    pub fn mark_in_progress_task(id: u32) {
        let mut tasks = Self::load_tasks();
        let task = tasks.iter_mut().find(|task| task.id == id);

        match task {
            Some(task) => {
                task.status = Status::InProgress;
                task.updated_at = Local::now();
            }
            None => {
                eprintln!("Task with ID {} not found.", id);
            }
        }
    }

    pub fn mark_done(id: u32) {
        let mut tasks = Self::load_tasks();
        let task = tasks.iter_mut().find(|task| task.id == id);

        match task {
            Some(task) => {
                task.status = Status::Done;
                task.updated_at = Local::now();
            }
            None => {
                eprintln!("Task with ID {} not found.", id);
            }
        }
    }

    pub fn restart_task(id: u32) {
        let mut tasks = Self::load_tasks();
        let task = tasks.iter_mut().find(|task| task.id == id);

        match task {
            Some(task) => {
                task.status = Status::Todo;
                task.updated_at = Local::now();
            }
            None => {
                eprintln!("Task with ID {} not found.", id);
            }
        }
    }

    pub fn list_tasks(filter: Option<String>) {
        let tasks = Self::load_tasks();

        let filtered_tasks = match filter {
            Some(filter) => match filter.as_str() {
                "all" => tasks.clone(),
                "done" => tasks
                    .iter()
                    .filter(|task| task.status == Status::Done)
                    .cloned()
                    .collect(),
                "in-progress" => tasks
                    .iter()
                    .filter(|task| task.status == Status::InProgress)
                    .cloned()
                    .collect(),
                "not-done" => tasks
                    .iter()
                    .filter(|task| task.status != Status::Done)
                    .cloned()
                    .collect(),
                _ => {
                    eprintln!("Usage: task-tracker-cli list [all | done | in-progress | not-done], default is all.");
                    return;
                }
            },
            None => tasks.clone(),
        };

        if tasks.is_empty() {
            println!("No tasks found.");
            return;
        }

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
