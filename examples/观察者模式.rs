use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
struct ThreadPool {
    threads: Arc<Mutex<Vec<i32>>>,
    num: i32,
    active: bool,
}
impl ThreadPool {
    fn new(vec: Vec<i32>, num: i32) -> ThreadPool {
        ThreadPool {
            threads: Arc::new(Mutex::new(vec)),
            num,
            active: false,
        }
    }
    fn set_extend_data(&self, data: Vec<i32>) {
        self.threads.lock().unwrap().extend(data);
    }
    fn set_data(&self, data: i32) {
        self.threads.lock().unwrap().push(data);
    }
    fn run(&self) {
        let data_clone = Arc::clone(&self.threads);
        let (tx, rx) = mpsc::channel();
        loop {
            data_clone
        }
    }
}
fn main() {
    // 创建一个可变的 Vec 并包装在 Arc 和 Mutex 中
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data_clone = Arc::clone(&data);
    // 创建通道
    let (tx, rx) = mpsc::channel();
    // 启动等待通知的线程
    let handle = thread::spawn(move || {
        println!("11212121212");
        // 等待通知
        for notification in rx {
            println!("Received notification: {}", notification);
            // 执行函数
            let data = data_clone.lock().unwrap();
            println!("11212121212{:?}", data);
        }
    });

    // 创建多个线程
    let mut handles = vec![];
    for i in 0..5 {
        let data_clone = Arc::clone(&data);
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            // 获取 Mutex 的锁并修改 Vec
            let mut vec = data_clone.lock().unwrap();
            vec.push(i);
            thread::sleep(Duration::from_secs(2));
            println!("Thread {} pushed value: {:?}", i, vec);
            tx.send(i).expect("这个是错误");
        });
        handles.push(handle);
    }
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    handle.join().unwrap();
    drop(tx);

    // 打印最终的 Vec
    println!("Final Vec: {:?}", data);
}
