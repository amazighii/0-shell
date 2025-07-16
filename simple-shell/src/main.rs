mod commands;
mod parser;
use once_cell::sync::Lazy;
use parser::parser_fn;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};

pub static CTRL_C_HIT: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

fn main() {
    ctrlc::set_handler(move || {
        CTRL_C_HIT.store(true, Ordering::SeqCst);
        // println!("im in main");
        // return;
    })
    .expect("Error setting ctrl + c handler");

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        let stdin = io::stdin();

        match stdin.read_line(&mut line) {
            Ok(n) if n == 0 => {
                println!("EOF: You are exiting 0-shell");
                break;
            }
            Err(err) => println!("Error: {err}"),
            Ok(_) => line = complete_line(&mut line),
        }

        parser_fn(line);
    }
}

fn complete_line(l: &mut String) -> String {
    let mut line = String::new();

    while !has_two_quotes(&l) {
        print!("> ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        stdin.read_line(&mut line).unwrap();
        l.push_str(line.as_str());
        line.clear();
    }

    l.clone()
}

fn has_two_quotes(line: &str) -> bool {
    let mut chars = line.chars().peekable();
    let mut count = 0;

    while let Some(c) = chars.next() {
        if c == '\\' && matches!(chars.peek(), Some('"')) {
            chars.next();
            continue;
        } else if c == '"' {
            count += 1;
        }
    }
    count % 2 == 0
}
