fn main() {
    let hello = String::from("hello");

    let mut s = String::from("foo");
    s.push_str("bar");
    let s2 = "bar2";
    s.push_str(s2);
    println!("s is {}", s);
    println!("s2 is {}", s2);
    let mut s3 = String::from("lo");
    s3.push('l');
    println!("s3 is {}", s3);

    let a1 = String::from("Hello, ");
    let a2 = String::from("world!");
    let a3 = a1 + &a2;
    println!("a3 is {}", a3);
    println!("a2 is {}", a2);

    let b1 = String::from("tic");
    let b2 = String::from("tac");
    let b3 = String::from("toc");
    let b = format!("{}-{}-{}", b1, b2, b3);
    println!("b is {}", b);

    let len = String::from("Hola").len();
    println!("len is {}", len);

    for c in "aAsScC".bytes() {
        println!("{}", c);
    }
    for c in "asc".chars() {
        println!("{}", c);
    }
}