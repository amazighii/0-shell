mod parser;
mod commands;
use std::io::{ self, Write };
use parser::parser_fn;


fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut line).unwrap();
        // println!("line: {}$", line.len());
        parser_fn(line);
    }
}
