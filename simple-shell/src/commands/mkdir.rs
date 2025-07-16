use std::fs::create_dir;
use std::io::ErrorKind;
pub fn mkdir(argument: Vec<&str>) {
    if argument.is_empty() {
        println!("mkdir: missing operand");
        return;
    }

    for path in argument.iter() {
        match create_dir(path) {
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
