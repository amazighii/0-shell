use std::fs;
use std::io::ErrorKind;

pub fn touch(argument: Vec<&str>) {
    if argument.is_empty() {
        println!("touch: missing file operand");
        return;
    }

    for path in argument.iter() {
        match fs::File::create(path) {
            Ok(_) => (),
         Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => {
                    println!("mkdir: {}: Folder exists", path);
                }
                ErrorKind::PermissionDenied => {
                    println!("mkdir: {}: Permission denied", path);
                }
                ErrorKind::NotFound => {
                    println!("mkdir: {}: No such file or directory", path);
                }
                _ => {
                    println!("mkdir: {}: {}", path, e);
                }
            },
        }
    }
}