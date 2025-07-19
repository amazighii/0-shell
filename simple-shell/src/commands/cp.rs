use std::{
    fs::{copy, metadata},
    io::ErrorKind,
};

pub fn cp(argument: Vec<&str>) {
    if argument.is_empty() {
        println!("Error: cp: missing file operand");
        return;
    } else if argument.len() == 1 {
        println!(
            "Error: cp: missing destination file operand after '{}'",
            argument[0]
        );
        return;
    }

    let to = argument[argument.len() - 1];

    for (i, file) in argument.iter().enumerate() {
        if i == argument.len() - 1 {
            break;
        }

        match metadata(to) {
            Ok(m) => {
                if m.is_dir() {
                    match copy(file, format!("{}/{}", to, file)) {
                        Ok(_) => (),
                        Err(err) => display_errors(err.kind()),
                    };
                } else if m.is_file() && argument.len() == 2 {
                    if to == *file {
                        println!("Error: cannot copy file to it self");
                        return;
                    }
                    match copy(file, to) {
                        Ok(_) => (),
                        Err(err) => display_errors(err.kind()),
                    };
                } else {
                    println!("Error: cp: cannot copy arguments to destination");
                    return;
                }
            }
            Err(err) => display_errors(err.kind()),
        }
    }
}

fn display_errors(err: ErrorKind) {
    match err {
        ErrorKind::NotFound => {
            println!("Error: cannot find the file or directory specified");
        }
        ErrorKind::PermissionDenied => {
            println!("Error: permission denied");
        }
        ErrorKind::InvalidInput => {
            println!("Error: invalid input provided");
        }
        ErrorKind::InvalidData => {
            println!("Error: the file contains invalid data");
        }
        ErrorKind::WriteZero => {
            println!("Error: failed to write the complete file");
        }
        ErrorKind::UnexpectedEof => {
            println!("Error: unexpected end of file");
        }
        ErrorKind::BrokenPipe => {
            println!("Error: broken pipe while copying");
        }
        ErrorKind::Other => {
            println!("Error: an unknown I/O error occurred - {}", err);
        }
        _ => {
            println!("Error: unhandled error occurred - {}", err);
        }
    }
}
