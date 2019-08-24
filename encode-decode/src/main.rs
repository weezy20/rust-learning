const ALPHABET_SIZE: u32 = 26;

fn main() {
    let message = scan_stdin_line();
    let key = scan_stdin_line().parse::<u32>().unwrap();
    let encoded_messsage = encode(message, key);
    println!("{}", encoded_messsage);
}

fn encode(msg: String, key: u32) -> String {
    let mut encoded = String::new();
    for c in msg.chars().into_iter() {
        let new_char = shift(c, key);
        encoded.push(new_char);
    }
    encoded
}

fn shift(item: char, key: u32) -> char {
    if !item.is_alphabetic() {
        return item;
    }
    let starting_letter = 'a' as u32;
    let actual_position = item as u32 - starting_letter;
    let new_shift = (actual_position + key) % ALPHABET_SIZE;
    std::char::from_u32(starting_letter + new_shift).unwrap()
}

fn scan_stdin_line() -> String {
    let mut message = String::new();
    std::io::stdin().read_line(&mut message).unwrap();
    message.trim().to_string()
}