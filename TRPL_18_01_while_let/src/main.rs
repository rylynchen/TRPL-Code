fn main() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(n) = stack.pop() {
        println!("{}", n);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} = {}", index, value);
    }
    for i in v.iter() {
        println!("{}", i);
    }
}