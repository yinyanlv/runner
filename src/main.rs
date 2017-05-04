use std::thread;
use std::sync::mpsc;

fn main() {

    let (sender, receiver) = mpsc::channel(); 

    let thread_1 = thread::spawn(move || {

        sender.send(1).unwrap();
    });

    thread_1.join();

    println!("main thread receive {:?}", receiver.recv().unwrap());
}