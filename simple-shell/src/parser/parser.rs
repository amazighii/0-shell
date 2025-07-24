#[derive(Debug)]
struct Input<'a> {
    command: String,
    flag: String,
    argument: Vec<&'a str>,
}

impl<'a> Input<'a> {
    fn new() -> Self {
        Self {
            command: "".to_string(),
            flag: "".to_string(),
            argument: vec![],
        }
    }

    fn parse_input(line: &'a str) -> Option<Self> {
        let res: Vec<&str> = line.split_whitespace().collect();
        let mut input = Input::new();

        if res.len() < 1 {
            return None;
        }

        input.command = res[0].to_string();

        if res.len() == 1 {
            return Some(input);
        }

        if res[0] == "echo" {
            input.argument = vec![line];
            return Some(input);
        }


        // println!("res: {:?}", res);

        for (i, v) in res.iter().enumerate() {
            if i == 0 {
                continue;
            }

            if v.starts_with("-") {
                input.flag.push_str(v);
                continue;
            }

            input.argument.push(v);
        }

        Some(input)
    }
}

pub fn parser_fn(line: String) {
    let input = Input::parse_input(&line);

    match input {
        Some(v) => check_command(v),
        None => return,
    }
}

use std::vec;

use crate::commands::{
    cat::cat, cd::cd, cp::cp, echo::echo, exit::exit, mkdir::mkdir, mv::mv, pwd::pwd, rm::rm,
    touch::touch,
};

fn check_command(input: Input) {
    match input.command.as_str() {
        "echo" => echo(input.argument),
        "cat" => cat(input.argument),
        "mkdir" => mkdir(input.argument),
        "cd" => cd(input.argument),
        "cp" => cp(input.argument),
        "mv" => mv(input.argument),
        "rm" => rm(input.argument, &input.flag),
        "touch" => touch(input.argument),
        "pwd" => pwd(),
        "exit" => exit(),
        _ => println!("Command '{}' not found", input.command),
    }
}
