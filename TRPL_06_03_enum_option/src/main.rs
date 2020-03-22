use std::option::Option;
use std::option::Option::None;
use std::option::Option::Some;

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    println!("six : {:?}", six);
    let none = plus_one(None);
    println!("none : {:?}", none);
}

fn plus_one(x: Option<u8>) -> Option<u8> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}