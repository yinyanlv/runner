use std::thread;
use std::sync::mpsc;

fn main() {

    let (sender, receiver): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

    thread::Builder::new()
                        .name("thread_1".to_string())
                        .stack_size(1024 * 1024 * 5)
                        .spawn(move || {
                            sender.send(111).unwrap();
                        }).unwrap();

    println!("{:?}", receiver.recv().unwrap());
}