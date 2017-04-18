extern crate iron;

use std::thread;
use std::sync::mpsc;

use iron::prelude::*;
use iron::status;

fn main() {

    let (sender, receiver): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

    thread::spawn(move || {
        println!("before action");
        sender.send(123).unwrap();
        println!("after action");
    });

    // Iron::new(|_: &mut Request| {
    //     Ok(Response::with((
    //         status::Ok,
    //         "Hello Iron"
    //     )))
    // }).http("localhost:3000").unwrap();

    println!("prev");
    println!("recevie {}", receiver.recv().unwrap());
    println!("next");
}
