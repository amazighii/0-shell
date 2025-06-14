mod parser;
use std::io::{ self, Write };
use parser::parser_fn;


fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut line).unwrap();
        parser_fn(line);
    }
}
