fn main() {
    let v1 = vec![2,4,6];
//    for item in v1 {
//        println!("Got {}",item);
//    }

    let v2 : Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);
}
