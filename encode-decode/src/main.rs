fn main() {
    let action = scan_stdin_line();
    let message = scan_stdin_line();
    let key = scan_stdin_line().parse::<i32>().unwrap();
    let encoded_messsage = match action.as_str() {
        "enc" => encode_decode(message, key),
        "dec" => encode_decode(message, -key),
        _ => String::new()
    };
    println!("{}", encoded_messsage);
}

fn encode_decode(msg: String, key: i32) -> String {
    let mut encoded = String::new();
    for c in msg.chars().into_iter() {
        let new_char = shift(c, key);
        encoded.push(new_char);
    }
    encoded
}

fn shift(item: char, key: i32) -> char {
    let starting_letter = 'a' as i32;
    let actual_position = item as i32 - starting_letter;
    let new_shift = actual_position as i32 + key;
    std::char::from_u32((starting_letter + new_shift) as u32).unwrap()
}

fn scan_stdin_line() -> String {
    let mut message = String::new();
    std::io::stdin().read_line(&mut message).unwrap();
    message.trim().to_string()
}