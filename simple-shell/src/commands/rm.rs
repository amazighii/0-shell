use std::fs::{remove_dir_all, remove_file};
use std::io::{self, Write};

pub fn rm(argument: Vec<&str>, flag: &str) {
    if argument.is_empty() {
        println!("rm: missing operand");
        return;
    }

        io::stdout().flush().unwrap();
        let mut line = String::new();
        let stdin = io::stdin();
        println!("Are you sure you want to remove this file? (yes/no):");

        match stdin.read_line(&mut line) {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err),
        }

        if line.trim() == "no" {
            return;
        } else if line.trim() != "yes" {
            return;
        }

    if flag == "-r" {
        for v in argument.iter() {
            match remove_dir_all(v) {
                Ok(_) => (),
                Err(_) => rm_file(argument.clone()),
            }
        }

        return;
    } else if !flag.is_empty() {
        println!("rm: unknown option {}", flag);
        return;
    }

    rm_file(argument);
}

fn rm_file(argument: Vec<&str>) {
    for v in argument.iter() {
        match remove_file(v) {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err),
        }
    }
}
