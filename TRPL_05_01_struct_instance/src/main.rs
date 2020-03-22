use std::string::String;

struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let mut user = build_user(String::from("rylynchen@gmail.com"), String::from("rylyn"));
    println!("email: {}", user.email);
    user.email = String::from("test@gmail.com");
    println!("email: {}", user.email);
    let user2 = User {
        username: String::from("test2"),
        ..user
    };
    println!("u2 username: {} email: {}", user2.username, user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
