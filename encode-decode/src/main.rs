fn main() {
    let message = "we found a treasure!";
    for c in message.chars() {
        let new_char = if c.is_alphabetic() {
            'z' as u32 - (c as u32 - 'a' as u32)
        } else {
            c as u32
        };
        print!("{}", std::char::from_u32(new_char).unwrap());
    }
}
