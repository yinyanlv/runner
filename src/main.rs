use std::thread;
use std::sync::{Arc, Mutex, Condvar};

fn main() {
    let arc = Arc::new((Mutex::new(false), Condvar::new()));
    let arc_temp = arc.clone();

    thread::spawn(move || {

        let (ref lock, ref condvar) = *arc_temp;
        let mut state = lock.lock().unwrap();

        *state = true;

        condvar.notify_one();

        println!("thread {}", *state);
    });

    let (ref lock, ref condvar) = *arc;
    let mut state = lock.lock().unwrap();

    println!("main {}", *state);

    while !*state {
        println!("main prev {}", *state);
        state = condvar.wait(state).unwrap();
        println!("main next {}", *state);
    }
}

