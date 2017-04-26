use std::thread;
use std::sync::{Arc, Mutex, Condvar};

fn main() {

    let wrapper = Arc::new((Mutex::new(false), Condvar::new()));
    let wrapper_temp = wrapper.clone();


    let (ref lock, ref cvar) = *wrapper;   
    let mut state = lock.lock().unwrap();

    thread::spawn(move || {
        let (ref lock, ref cvar) = *wrapper_temp;
        let mut state = lock.lock().unwrap();

        *state = true;
        cvar.notify_one();
        println!("notify main");
    });

    while !*state {
        println!("before main wait");

        state = cvar.wait(state).unwrap();

        println!("after main wait");
    }
}