fn main() {
    let p1 = Point { x: 5, y: 10.04 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

//fn largest_i32(list: &[i32]) -> i32 {
//    let mut largest = list[0];
//    for &item in list.iter() {
//        if (item > largest) {
//            largest = item;
//        }
//    }
//    largest
//}
//
//fn largest_char(list: &[char]) -> char {
//    let mut largest = list[0];
//    for &item in list.iter() {
//        if (item > largest) {
//            largest = item;
//        }
//    }
//    largest
//}