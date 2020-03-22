fn main() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    let z = plus_one(5);
    println!("The value of x is {}.", x);
    println!("The value of y is {}.", y);
    println!("The value of z is {}.", z);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}