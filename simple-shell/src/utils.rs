use std::io::{self, Write};

pub fn complete_line(l: &mut String) -> String {
    let mut line = String::new();

    while !has_two_quotes(&l) {
        print!("> ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        stdin.read_line(&mut line).unwrap();
        l.push_str(line.as_str());
        line.clear();
    }

    l.clone()
}

pub fn has_two_quotes(line: &str) -> bool {
    let mut chars = line.chars().peekable();
    let mut count = 0;

    while let Some(c) = chars.next() {
        if c == '\\' && matches!(chars.peek(), Some('"')) {
            chars.next();
            continue;
        } else if c == '"' {
            count += 1;
        }
    }
    count % 2 == 0
}
