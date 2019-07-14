use std::io::{stdin, stdout, Write};

pub fn read_line_with_prompt(prompt_str: &str) -> String {
    print!("{}", prompt_str);
    stdout().flush().unwrap();
    let mut str = String::new();
    stdin().read_line(&mut str).unwrap();
    str.trim().to_string()
}