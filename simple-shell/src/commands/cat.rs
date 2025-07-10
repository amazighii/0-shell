use std::fs::File;
use std::io::prelude::*;

pub fn cat(argument: Vec<&str>) {

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
