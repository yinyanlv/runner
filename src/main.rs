use std::thread;
use std::sync::{Arc, Mutex, Condvar};

fn main() {

    let wrapper = Arc::new((Mutex::new(false), Condvar::new()));
    let wrapper_clone = wrapper.clone();

    thread::Builder::new()
        .name("thread_1".to_string())
        .stack_size(4 * 1024 * 1024)
        .spawn(move || {
            let (ref lock, ref condition) = *wrapper_clone;    
            let mut state = lock.lock().unwrap();

            *state = true;

            condition.notify_one();
        })
        .unwrap(); 

    let (ref lock, ref condition) = *wrapper; 
    let mut state = lock.lock().unwrap();

    while !*state {

        state = condition.wait(state).unwrap();

        println!("receive {}", *state);
    }

    println!("-- main end --");
}