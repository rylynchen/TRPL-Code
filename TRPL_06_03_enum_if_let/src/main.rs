use std::option::Option::Some;

fn main() {
    let one = Some(1);
    let two = match one {
        Some(1) => {
            Some(2)
        }
        _ => Some(1),
    };
    println!("two : {:?}", two);
    let mut count = 0;
    if let two = Some(2) {
        count += 1;
    }
    println!("count : {}", count);
}