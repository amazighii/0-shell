use std::fs::File;
// use std::io::prelude::*;
use std::io::{self, prelude::*, Write};

pub fn cat(argument: Vec<&str>) {
    // println!("{:?}", argument);
    if argument.len() == 0 {
        // ctrlc::set_handler(move || {
        //     return;
        // })
        // .expect("Error setting ctrl + c handler");
        loop {
            io::stdout().flush().unwrap();
            let mut line = String::new();
            let stdin = io::stdin();
            stdin.read_line(&mut line).unwrap();
            print!("{}", line);
        }
    }

    for v in argument.iter() {
        let mut file = match File::open(v) {
            Ok(f) => f,
            Err(_) => {
                println!("cat: {}: Is not a file", v);
                continue;
            }
        };

        let mut contents = String::new();
        let _ = match file.read_to_string(&mut contents) {
            Ok(n) => n,
            Err(err) => {
                println!("{} {}", v, err);
                continue;
            }
        };

        print!("{}", contents);
    }
}
