fn main() {
    println!("Hello, world!");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("y: {}, z: {}", y,tup.2);
}
