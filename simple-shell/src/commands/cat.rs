use crate::CTRL_C_HIT;
use std::fs::File;
use std::io::{self, prelude::*, Write};
use std::sync::atomic::Ordering;
// use std::sync::Arc;
// use std::thread;
// use std::time::Duration;

pub fn cat(argument: Vec<&str>) {
    if argument.len() == 0 {
        CTRL_C_HIT.store(false, Ordering::SeqCst);

        loop {
            if CTRL_C_HIT.load(Ordering::SeqCst) {
                println!("Ctrl + C has been pressed");
                CTRL_C_HIT.store(false, Ordering::SeqCst);
                return;
            }

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
