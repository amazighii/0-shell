mod commands;
mod parser;
use once_cell::sync::Lazy;
use parser::parser_fn;
use std::env::current_dir;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
mod utils;
use crate::utils::*;

pub static CTRL_C_HIT: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

fn main() {
    ctrlc::set_handler(move || {
        CTRL_C_HIT.store(true, Ordering::SeqCst);
    })
    .expect("Error setting ctrl + c handler");

    loop {
        match current_dir() {
            Ok(path) => print!("{}$ ", path.display()),
            Err(err) => println!("pwd err: {}", err),
        }

        io::stdout().flush().unwrap();
        let mut line = String::new();
        let stdin = io::stdin();

        match stdin.read_line(&mut line) {
            Ok(n) if n == 0 => {
                println!("\nEOF: You are exiting 0-shell");
                break;
            }
            Err(err) => println!("Error: {err}"),
            Ok(_) => line = complete_line(&mut line),
        }

        parser_fn(line);
    }
}

