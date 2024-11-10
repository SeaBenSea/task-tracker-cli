# Task Tracker CLI

[![Build Status](https://github.com/seabensea/task-tracker-cli/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/seabensea/task-tracker-cli/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/seabensea/task-tracker-cli)](https://github.com/seabensea/task-tracker-cli/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple and efficient command-line interface (CLI) tool for managing and tracking tasks, written in Rust. `task-tracker-cli` allows you to add, update, delete, and list tasks directly from your terminal, with tasks stored persistently in a JSON file.

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
  - [From Pre-built Binaries](#from-pre-built-binaries)
    - [Windows](#windows)
    - [Linux and macOS](#linux-and-macos)
    - [Advanced: Adding to PATH](#advanced-adding-to-path)
  - [From Source](#from-source)
- [Usage](#usage)
  - [Commands](#commands)
  - [Examples](#examples)
- [Task Storage](#task-storage)
- [Built With](#built-with)
- [Platform Support](#platform-support)
- [Contributing](#contributing)
  - [Code Formatting and Linting](#code-formatting-and-linting)
- [Running Tests](#running-tests)
- [License](#license)
- [Credits](#credits)

## Features

- **Add Tasks**: Quickly add new tasks with descriptions.
- **Update Tasks**: Modify existing tasks.
- **Delete Tasks**: Remove tasks when they're no longer needed.
- **Task Status Management**: Mark tasks as "In Progress," "Done," or restart them back to "Todo."
- **Filter Tasks**: View tasks based on their status.
- **Persistent Storage**: Tasks are saved in a JSON file for persistence.

## Prerequisites

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install) if building from source.
- **Terminal**: A command-line interface to run the program.

## Installation

### From Pre-built Binaries

Pre-built binaries are available for **Windows**, **Linux**, and **macOS** on the [Releases](https://github.com/seabensea/task-tracker-cli/releases) page.

#### Windows

1. **Download the binary**:

   - Go to the [Releases](https://github.com/seabensea/task-tracker-cli/releases) page.
   - Download the `task-tracker-cli-release.zip` file.
   - Extract the contents of the ZIP file.

2. **Navigate to the download location** in Command Prompt or PowerShell:

   ```cmd
   cd C:\Path\To\Extracted\Folder\task-tracker-cli-windows
   ```

3. **Run the application** directly:

   ```cmd
   task-tracker-cli.exe
   ```

   You can now use the CLI by prefixing commands with `task-tracker-cli.exe`. For example:

   ```cmd
   task-tracker-cli.exe add "Finish Rust project"
   ```

#### Linux and macOS

1. **Download the binary**:

   - Go to the [Releases](https://github.com/seabensea/task-tracker-cli/releases) page.
   - Download the `task-tracker-cli-release.zip` file.
   - Extract the contents of the ZIP file.

2. **Navigate to the download location** in your terminal:

   ```bash
   cd /path/to/extracted/folder/task-tracker-cli-linux
   ```

3. **Make the binary executable**:

   ```bash
   chmod +x task-tracker-cli
   ```

4. **Run the application** directly:

   ```bash
   ./task-tracker-cli
   ```

   You can now use the CLI by prefixing commands with `./task-tracker-cli`. For example:

   ```bash
   ./task-tracker-cli add "Finish Rust project"
   ```

#### Advanced: Adding to PATH

For convenience, you can add the binary to a directory that's in your system's `PATH`, allowing you to run `task-tracker-cli` from anywhere without specifying the path.

**Windows:**

1. **Move the binary** to a directory in your `PATH`:

   - Copy or move the `.exe` file to `C:\Windows\System32` or any directory that's in your `PATH`.
   - Alternatively, add the directory containing `task-tracker-cli.exe` to your `PATH` environment variable.

2. **Verify installation**:

   Open a new Command Prompt or PowerShell window and run:

   ```cmd
   task-tracker-cli --help
   ```

**Linux and macOS:**

1. **Move the binary** to `/usr/local/bin/`:

   ```bash
   sudo mv task-tracker-cli /usr/local/bin/task-tracker-cli
   ```

2. **Verify installation**:

   Open a new terminal window and run:

   ```bash
   task-tracker-cli --help
   ```

### From Source

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/seabensea/task-tracker-cli.git
cd task-tracker-cli
```

Build the project in release mode:

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
  task-tracker-cli add "<description>"
  ```

- **Update an existing task**

  ```bash
  task-tracker-cli update <id> "<new_description>"
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

### Examples

#### Adding a Task

```bash
task-tracker-cli add "Finish Rust project"
```

**Output:**

```bash
Task added successfully with ID: 1
ID    | Description                              | Status          | Created At           | Updated At
---------------------------------------------------------------------------------------------------------------
1     | Finish Rust project                      | Todo            | 01-01-2023 01:23:45  | 01-01-2023 01:23:45
```

#### Listing All Tasks

```bash
task-tracker-cli list all
```

**Output:**

```bash
ID    | Description                              | Status          | Created At           | Updated At
---------------------------------------------------------------------------------------------------------------
1     | Finish Rust project                      | Todo            | 01-01-2023 01:23:45  | 01-01-2023 01:23:45
```

#### Updating a Task

```bash
task-tracker-cli update 1 "Finish Rust project with README"
```

**Output:**

```bash
Task with ID 1 updated successfully.
ID    | Description                              | Status          | Created At           | Updated At
---------------------------------------------------------------------------------------------------------------
1     | Finish Rust project with README          | Todo            | 01-01-2023 01:23:45  | 01-01-2023 01:24:45
```

#### Marking a Task as Done

```bash
task-tracker-cli mark-done 1
```

**Output:**

```bash
Task with ID 1 marked as done.
ID    | Description                              | Status          | Created At           | Updated At
---------------------------------------------------------------------------------------------------------------
1     | Finish Rust project with README          | Done            | 01-01-2023 01:23:45  | 01-01-2023 01:25:45
```

#### Deleting a Task

```bash
task-tracker-cli delete 1
```

**Output:**

```bash
Task with ID 1 deleted successfully.
No tasks found.
```

#### Viewing Help Information

```bash
task-tracker-cli help
```

**Output:**

```bash
Usage: task-tracker-cli <command> [arguments]
Commands:
  add <description> - Add a new task
  update <id> <description> - Update a task
  delete <id> - Delete a task
  mark-in-progress <id> - Mark a task as in progress
  mark-done <id> - Mark a task as done
  restart-task <id> - Restart a task
  list [all | done | in-progress | not-done] - List tasks
```

## Task Storage

Tasks are stored in a `tasks.json` file located in the current directory. Each task contains:

- **ID**: Unique identifier.
- **Description**: Details of the task.
- **Status**: Can be `Todo`, `In Progress`, or `Done`.
- **Created At**: Timestamp when the task was created.
- **Updated At**: Timestamp of the last update to the task.

## Built With

- [Rust](https://www.rust-lang.org/) - The programming language used.
- [Serde](https://crates.io/crates/serde) - For serialization and deserialization of data.
- [Chrono](https://crates.io/crates/chrono) - For date and time handling.
- [Colored](https://crates.io/crates/colored) - For colored terminal output.

## Platform Support

`task-tracker-cli` supports the following operating systems:

- **Linux**
- **macOS**
- **Windows**

## Contributing

Contributions are welcome! Please follow these steps:

1. **Fork the repository** on GitHub.
2. **Clone your fork** locally:

   ```bash
   git clone https://github.com/seabensea/task-tracker-cli.git
   ```

3. **Create a new branch** for your feature or bugfix:

   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Commit your changes** with clear messages:

   ```bash
   git commit -am "Add new feature"
   ```

5. **Push to your fork**:

   ```bash
   git push origin feature/your-feature-name
   ```

6. **Submit a pull request** to the main repository.

### Code Formatting and Linting

Ensure your code is properly formatted and free of common issues:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

## Running Tests

To run the tests for this project, use the following command:

```bash
cargo test
```

## License

This project is licensed under the MIT License.

## Credits

This project is inspired by the [Task Tracker challenge](https://roadmap.sh/projects/task-tracker) on [roadmap.sh](https://roadmap.sh/).
