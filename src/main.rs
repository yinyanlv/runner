use std::thread;
use std::sync::mpsc;

fn main() {

    let (sender, receiver) = mpsc::channel();

    let temp = thread::spawn(move || {
        let value = "abc";

        sender.send(value).unwrap();

        println!("child thread send {}", value);
    });

    temp.join().unwrap();

    let res = receiver.recv().unwrap();

    println!("main thread receive {}", res);

    println!("-- main end --");
}