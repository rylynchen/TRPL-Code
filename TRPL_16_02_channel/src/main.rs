use std::sync::mpsc::{channel, Sender};
use std::thread::spawn;

fn main() {
    let (tx, rx) = channel();
    let handle = spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
//        println!("val is {}", val);
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let (tx0, rx0) = channel();
    let tx1 = Sender::clone(&tx0);
    spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx0.send(val).unwrap();
        }
    });
    spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
        }
    });
    for val in rx0 {
        println!("Got {}", val);
    }
}
