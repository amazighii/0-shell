pub fn echo(argument: Vec<&str>) {
    let mut chars = argument[0].trim_end().chars().skip(5).peekable();
    let mut resu = vec![];
    let mut is_quote = false;
    let mut current = String::new();

    while let Some(c) = chars.next() {
        match c {
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
