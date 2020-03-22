use std::string::String;

fn main() {
    let s = String::from("hello"); // s into

    take_ownership(s); // s out : move

    let x = 5; // x into

    make_copy(x); // x not out because of base type
} // x out

fn take_ownership(some_thing: String) {
    println!("take ownership : {}", some_thing);
}

fn make_copy (y: u32) {
    println!("make copy : {}", y);
}