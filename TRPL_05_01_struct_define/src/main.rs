use std::string::String;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println!("Hello, world!");
    let user1 = User {
        email: String::from("rylynchen@gmail.com"),
        username: String::from("rylyn"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1.email);
}
