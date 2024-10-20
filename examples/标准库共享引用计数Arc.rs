use std::sync::Arc;
use std::thread;

fn main() {
    let apple = Arc::new("the same apple");
    for _ in 0..10 {
        let apple = Arc::clone(&apple);
        thread::spawn(move || {
            println!("{:?}", apple);
        });
    }
}
