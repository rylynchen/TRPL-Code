fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    let third: &i32 = &v[2];
    println!("v {:?}", v);
    println!("The third is {}", third);
    let mut v2 = vec![1, 2, 3];
    v2.push(4);
    println!("v2 {:?}", v2);
    match v2.get(2) {
        Some(third) => println!("The third is {}", third),
        None => println!("There is no third"),
    }
    for i in &mut v2 {
        *i += 50;
    }
    println!("v2 {:?}", v2);
}
