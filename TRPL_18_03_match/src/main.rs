fn main() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("x = 50"),
        Some(y) => println!("x = {}", y),
        _ => {}
    }
    println!("end, x = {:?}, y = {:?}", x, y);

    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("x, y:0"),
        Point { x: 0, y } => println!("x:0, y"),
        Point { x, y } => println!("x, y"),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move {}, {}", x, y),
        Message::Write(x) => println!("Write {}", x),
        Message::ChangeColor(Color::Rgb(x,y,z)) => println!("Change color rgb : {}, {}, {}", x, y, z),
        Message::ChangeColor(Color::Hsv(x,y,z)) => println!("Change color hsv : {}, {}, {}", x, y, z),
    }

    let x = Some(10);
    let y = 10;
    match x {
        Some(50) => println!("50"),
        Some(n) if n == y => println!("{}==y", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 6;
    let y = false;
    match x {
        4|5|6 if y => println!("y"),
        _ => println!("n"),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

struct Point {
    x: i32,
    y: i32,
}