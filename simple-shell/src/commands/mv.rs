use std::{env::current_dir, fs::rename};
use std::path::Path;

pub fn mv(argument: Vec<&str>) {
    if argument.is_empty() {
        println!("mv: missing operand");
        return;
    } else if argument.len() == 1 {
        println!("mv: missing destination operand after {}", argument[0]);
        return;
    }

    let dis = argument[argument.len() - 1];

    for (i, v) in argument.iter().enumerate() {
        if i == argument.len() - 1 {
            break;
        }

        let mut to = format!("{}/{}", dis, v);

        //still need to handl the dot case
        if dis == "." {
            match current_dir() {
                Ok(p) => {
                    let path = Path::new(&p);

                    match path.parent() {
                        Some(parent) => to = format!("{}", parent.display()),
                        None => println!("No parent directory"),
                    }
                },
                Err(err) => println!("Error: {}", err),
            }
        }

        println!("v: {v}, to: {to}");
        match rename(v, to) {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err),
        }
    }
}

