mod models;
mod task_manager;

use std::env;
use task_manager::TaskManager;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: task-tracker-cli <command> [arguments]");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() != 3 {
                eprintln!("Usage: task-tracker-cli add <description>");
                return;
            }

            let task_name = &args[2];
            TaskManager::add_task(task_name.to_string());
        }
        "update" => {
            if args.len() != 4 {
                eprintln!("Usage: task-tracker-cli update <id> <description>");
                return;
            }

            let task_id = args[2].parse::<u32>().unwrap();
            let task_description = args[3].clone();
            TaskManager::update_task(task_id, task_description);
        }
        "delete" => {
            if args.len() != 3 {
                eprintln!("Usage: task-tracker-cli delete <id>");
                return;
            }

            let task_id = args[2].parse::<u32>().unwrap();
            TaskManager::delete_task(task_id);
        }
        "mark-in-progress" => {
            if args.len() != 3 {
                eprintln!("Usage: task-tracker-cli mark-in-progress <id>");
                return;
            }

            let task_id = args[2].parse::<u32>().unwrap();
            TaskManager::mark_in_progress_task(task_id);
        }
        "mark-done" => {
            if args.len() != 3 {
                eprintln!("Usage: task-tracker-cli mark-done <id>");
                return;
            }

            let task_id = args[2].parse::<u32>().unwrap();
            TaskManager::mark_done(task_id);
        }
        "restart-task" => {
            if args.len() != 3 {
                eprintln!("Usage: task-tracker-cli restart-task <id>");
                return;
            }

            let task_id = args[2].parse::<u32>().unwrap();
            TaskManager::restart_task(task_id);
        }
        "list" => TaskManager::list_tasks(args.get(2).cloned()),
        "help" => {
            println!("Usage: task-tracker-cli <command> [arguments]");
            println!("Commands:");
            println!("  add <description> - Add a new task");
            println!("  update <id> <description> - Update a task");
            println!("  delete <id> - Delete a task");
            println!("  mark-in-progress <id> - Mark a task as in progress");
            println!("  mark-done <id> - Mark a task as done");
            println!("  restart-task <id> - Restart a task");
            println!("  list [all | done | in-progress | not-done] - List tasks");
        }
        _ => {
            eprintln!("Unknown command. Please use 'help' to see available options.");
        }
    }
}
