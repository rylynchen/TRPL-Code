use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let v = vec![1,2,3];
    let handle = spawn(move||{
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
//    let handle =  spawn(|| {
//        for i in 1..10 {
//            println!("hi number {} from the spawned thread!", i);
//            sleep(Duration::from_millis(1));
//        }
//    });
//
//    for i in 1..5 {
//        println!("hi number {} from the main thread!", i);
//        sleep(Duration::from_millis(1));
//    }
//    println!("main end");
//    handle.join().unwrap_or_default();
//    println!("end");
}
