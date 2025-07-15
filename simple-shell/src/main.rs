mod commands;
mod parser;
use parser::parser_fn;
use std::io::{self, Write};

        let mut is_ctrlc_pressed = false;
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        let stdin = io::stdin();


        ctrlc::set_handler(move || {
            is_ctrlc_pressed = true;
        })
        .expect("Error setting ctrl + c handler");

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
