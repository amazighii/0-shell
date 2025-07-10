#[derive(Debug)]
struct Input {
    command: String,
    flag: String,
    argument: String,
}

impl Input {
    fn parse_input(line: String) -> Option<Self> {
        let res: Vec<&str> = line.split_whitespace().collect();

        if res.len() < 1 {
            return None;
        }

        if res.len() == 1 {
            return Some(Self {
                command: res[0].to_string(),
                flag: "".to_string(),
                argument: "".to_string(),
            });
        }

        if res[0] == "echo" {
            return Some(Self {
                command: res[0].to_string(),
                flag: "".to_string(),
                argument: res[1..].join(" "),
            });
        }

        for (i, v) in res.iter().enumerate() {
            if i == 0 {
                continue;
            }

            // check for flags and arguments
        }

        Some(Self {
            command: res[0].to_string(),
            flag: "".to_string(),
            argument: "".to_string(),
        })
    }
}

pub fn parser_fn(line: String) {
    let mut input = Input::parse_input(line);

    match input {
        Some(v) => check_command(v),
        None => return,
    }
}

use crate::commands::echo::echo;

fn check_command(input: Input) {
    match input.command.as_str() {
        "echo" => echo(input.argument),
        _ => println!("Command '{}' not found", input.command),
    }
}
