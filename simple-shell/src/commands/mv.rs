use std::io::ErrorKind;
use std::path::Path;
use std::{env::current_dir, fs::rename};

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

        //still need to handle the dot case
        // if dis == "." {
        //     match current_dir() {
        //         Ok(p) => {
        //             let path = Path::new(&p);

        //             match path.parent() {
        //                 Some(parent) => to = format!("{}", parent.display()),
        //                 None => println!("No parent directory"),
        //             }
        //         },
        //         Err(err) => println!("Error: {}", err),
        //     }
        // }

        let  to = format!("{}/{}", dis, v);

        // println!("v: {v}, to: {to}");
        // println!("v: {v}, dis: {dis}");
        match rename(v, &to) {
            Ok(_) => (),
            Err(_) => {
                let vv = Path::new(v);
                let to_to = Path::new(&dis);
                // println!("exists: vv: {}, to_to: {}", vv.exists(), to_to.exists());

                if vv.exists() && to_to.exists() {
                    if vv.is_file() && to_to.is_file() {
                        match rename(v, dis) {
                            Ok(_) => (),
                            Err(err) => error(err.kind()),
                        }
                    } else {
                        println!(
                            "mv: cannot overwrite non-directory '{}' with directory '{}' ",
                            dis, v
                        );
                    }
                } else if !to_to.exists() {
                        match rename(v, dis) {
                            Ok(_) => (),
                            Err(err) => error(err.kind()),
                        }
                } else {
                    println!("mv: source file not found");
                }
            }
        }
    }
}

fn error(err: ErrorKind) {
    use std::io::ErrorKind;

    match err {
        ErrorKind::NotFound => eprintln!("mv: source file not found"),
        ErrorKind::PermissionDenied => eprintln!("mv: permission denied"),
        ErrorKind::AlreadyExists => eprintln!("mv: destination already exists"),
        ErrorKind::InvalidInput => eprintln!("mv: invalid input"),
        _ => eprintln!("mv: unknown error: {err}"),
    }
}
// Not working:
//mv test2 test1/test2
