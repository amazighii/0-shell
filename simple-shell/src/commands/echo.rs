pub fn echo(argument: Vec<&str>) {
    if argument.is_empty() {
        println!();
        return;
    }

    let line = argument[0];
    let mut chars = line.trim_end().chars().skip(5).peekable();
    let mut resu = vec![];
    let mut is_quote = false;
    let mut current = String::new();

    while let Some(c) = chars.next() {
        match c {
            '\\' if matches!(chars.peek(), Some('"')) => {
                current.push(chars.next().unwrap());
            }
            '"' => {
                is_quote = !is_quote;
                continue;
            }
            ' ' if !is_quote => {
                if !current.is_empty() {
                    resu.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        resu.push(current.clone());
    }
    println!("{}", resu.join(" "));
}
