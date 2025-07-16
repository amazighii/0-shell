use std::{
    env::{set_current_dir, var},
    io::ErrorKind,
};

pub fn cd(argument: Vec<&str>) {
    match argument.len() {
        0 => match var("HOME") {
            Ok(val) => println!("{}", val),
            Err(_) => match var("USERPROFILE") {
                Ok(val) => println!("{}", val),
                Err(err) => println!("Err: {}", err),
            },
        },
        1 => match set_current_dir(argument[0]) {
            Ok(_) => (),
            Err(err) => handle_cd_error(&err, argument[0]),
        },
        _ => eprintln!("cd: too many arguments"),
    };
}

use std::io;
// use std::io::ErrorKind;

fn handle_cd_error(err: &io::Error, path: &str) {
    match err.kind() {
        ErrorKind::NotFound => {
            eprintln!("cd: no such file or directory: {}", path);
        }
        ErrorKind::PermissionDenied => {
            eprintln!("cd: permission denied: {}", path);
        }
        ErrorKind::InvalidInput => {
            eprintln!("cd: invalid path: {}", path);
        }
        ErrorKind::NotADirectory => {
            eprintln!("cd: {}: is not a directory", path);
        }
        _ => {
            eprintln!("cd: failed to change directory: {}", err);
        }
    }
}
