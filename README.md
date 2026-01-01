# 0-Shell

## Overview

0-Shell is a minimalist Unix-like shell written in Rust. It executes core shell commands using system-level Rust abstractions without relying on external binaries or existing shells such as bash or sh.

Inspired by tools like BusyBox, this project focuses on understanding how a shell works internally: command parsing, filesystem interaction, and process control, while benefiting from Rust’s safety and abstraction features.

---

## Learning Objectives

- Work with file and directory operations
- Implement an interactive shell loop
- Parse and execute user input
- Implement robust error handling
- Gain experience with Unix process and system call APIs in Rust

---

## Core Requirements

The shell must:

- Display a prompt (`$ `) and wait for user input
- Parse and execute user commands
- Return to the prompt only after command execution completes
- Exit gracefully on EOF (Ctrl+D)

All commands must be implemented from scratch using Rust system-level abstractions. External binaries or shells must not be used.

---

## Supported Commands

The shell implements the following commands:

- echo
- cd
- pwd
- cat
- cp
- rm (supports recursive deletion with -r)
- mv
- mkdir
- exit

If a command is not recognized, the shell prints:

Command '<name>' not found

---

## Constraints

- Only basic command
- No piping (|), redirection (>, <), or globbing (*)
- Shell behavior follows standard Unix conventions
- Code is following good coding practices

---

## Example Usage

student$ ./0-shell
$ cd dev
$ pwd
/dev
$ echo "Hello There"
Hello There
$ something
Command 'something' not found
$ exit
student$

---
## Project structure
```
📁 simple-shell
├── 📁 src
│ ├── main.rs
│ ├── lib.rs
│ ├── 📁 commands # ⬅ Each command's logic
│ │ ├── mod.rs
│ │ ├── echo.rs
│ │ ├── cd.rs
│ │ ├── ...
│ ├── 📁 parser # ⬅ Input tokenizer and syntax parser
│ │ ├── mod.rs
│ │ └── parser.rs
│ ├── executor.rs # ⬅ Matches parsed command to a handler
│ └── utils.rs
├── Cargo.toml
└── README.md
```

## How to Build and Run

### Prerequisites

- Rust toolchain installed (includes cargo)

Verify installation:

rustc --version
cargo --version

---

### Build

cargo build

---

### Run

Run using Cargo:

cargo run

---

## License

This project is for educational purposes. You are free to use, modify, and extend it.
