use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use std::thread;

lazy_static! {
    pub static ref COUNTER: Mutex<i32> = Mutex::new(0);
}

fn main() {
    let mut handles = vec![];

    for _ in 0..10 {
        // let counter = Arc::clone(&COUNTER);
        let handle = thread::spawn(move || {
            let mut num = COUNTER.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = COUNTER.lock().unwrap();
    println!("Final count: {}", *final_count);
}
