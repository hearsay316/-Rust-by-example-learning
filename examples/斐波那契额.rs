use std::thread;
use std::time::Instant;

fn fibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let start = Instant::now(); // 记录开始时间
    let mut threads = Vec::new();
    for i in 0..8 {
        let handle = thread::spawn(move || {
            let result = fibonacci(4000);
            println!("thread {i} result: {result}");
        });
        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }
    let duration = start.elapsed(); // 计算运行时间
    println!("Time elapsed in fibonacci() is: {:?}", duration);
}
