use std::thread;
use std::sync::mpsc;

fn main() {

    let (sender, recevier): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

    thread::spawn(move || {
        sender.send(32).unwrap();
    });

    let temp = recevier.recv();

    println!("revevie {}", temp.unwrap());
}