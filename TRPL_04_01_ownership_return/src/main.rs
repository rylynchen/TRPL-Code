use std::string::String;

fn main() {
    let s1 = gives_ownership(); // s1 into

    let s2 = String::from("hello"); // s2 into

    let s3 = takes_and_gives_back(s2); // s2 out, s3 into

    let (s4, length) = calculate_length(s3); // s3 out, s4 into , length into

    println!("The length of '{}' is {}.", s4, length);
} // s4 out ,length out

fn gives_ownership() -> String {
    let some_thing = String::from("hello"); // some_thing into
    some_thing  // some_thing out : move
}

fn takes_and_gives_back(a_string: String) -> String { // a_string into
    a_string    // a_string out : move
}

fn calculate_length(s : String) -> (String, usize) {
    let length = s.len();
    (s, length)
}