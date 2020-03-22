use std::string::String;

fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("s: {}", s);

    let r1 = &s;
    let r2 = &s;
    println!("r1 : {}, sr : {}.", r1, r2);
    let r3 = &mut s;
    println!("r3 : {}", r3);
}

fn change (some_thing: &mut String) {
    some_thing.push_str(", world");
}