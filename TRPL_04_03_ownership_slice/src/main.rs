use std::string::String;

fn main() {
    let mut s = String::from("hello world");
    let word = first_world(&s[..]);
    println!("s : {}, first word : {}", s, word);
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}