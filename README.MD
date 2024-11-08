# Task Tracker CLI

`task-tracker-cli` is a simple command-line interface (CLI) tool for managing and tracking tasks, written in Rust. The tool supports adding, updating, deleting, and listing tasks, which are stored in a JSON file for persistence.

## Features

- Add new tasks with descriptions
- Update existing tasks
- Delete tasks
- Mark tasks as "In Progress" or "Done"
- Restart tasks back to "Todo"
- Filter and view tasks by status

## Prerequisites

- [Rust](https://www.rust-lang.org/) (installation instructions available on the Rust website)
- A terminal or command line to run the program

## Installation

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/seabensea/task-tracker-cli.git
cd task-tracker-cli
```

Build the project:

```bash
cargo build --release
```

Run the project:

```bash
./target/release/task-tracker-cli
```

## Usage

To use the CLI tool, run it with the following commands:

### Commands

- **Add a new task**

  ```bash
  task-tracker-cli add <description>
  ```

- **Update an existing task**

  ```bash
  task-tracker-cli update <id> <new_description>
  ```

- **Delete a task**

  ```bash
  task-tracker-cli delete <id>
  ```

- **Mark a task as "In Progress"**

  ```bash
  task-tracker-cli mark-in-progress <id>
  ```

- **Mark a task as "Done"**

  ```bash
  task-tracker-cli mark-done <id>
  ```

- **Restart a task to "Todo" status**

  ```bash
  task-tracker-cli restart-task <id>
  ```

- **List tasks with optional filters**

  ```bash
  task-tracker-cli list [all | done | in-progress | not-done]
  ```

- **View help information**

  ```bash
  task-tracker-cli help
  ```

### Example

```bash
# Add a task
task-tracker-cli add "Finish Rust project"

# Update a task
task-tracker-cli update 1 "Finish Rust project with README"

# List all tasks
task-tracker-cli list all

# Mark a task as done
task-tracker-cli mark-done 1

# Delete a task
task-tracker-cli delete 1
```

## Task Storage

Tasks are stored in a `tasks.json` file located in the current directory. Each task contains:

- **ID**: Unique identifier
- **Description**: Details of the task
- **Status**: Status (Todo, In Progress, Done)
- **Created At**: Timestamp of creation
- **Updated At**: Timestamp of the last update

## Contributing

Contributions, issues, and feature requests are welcome. Feel free to check [issues](https://github.com/seabensea/task-tracker-cli/issues) and submit a PR.

## Credits

This project is inspired by the [Task Tracker challenge](https://roadmap.sh/projects/task-tracker) on [roadmap.sh](https://roadmap.sh/).

## License

This project is licensed under the MIT License.
