use std::sync::{Arc, Mutex};
use std::thread;
struct ThreadPool {
    threads: Arc<Mutex<usize>>,
    threads_data: Vec<i32>,
    num: usize,
}
impl ThreadPool {
    fn new(vec: Vec<i32>, num: usize) -> ThreadPool {
        let mut s = ThreadPool {
            threads_data: vec,
            threads: Arc::new(Mutex::new(0)),
            num,
        };
        ThreadPool::run(&mut s);
        s
    }
    fn set_extend_data(&mut self, data: Vec<i32>) {
        self.threads_data.extend(data);
        self.run();
    }
    fn run(&mut self) {
        let data_clone = Arc::clone(&self.threads);
        loop {
            let mut data_num = self.num;
            if self.threads_data.len() < data_num {
                data_num = self.threads_data.len();
            }
            let data: Vec<i32> = self.threads_data.drain(0..data_num).collect();
            let mut datum_spawn = vec![];
            for datum in data {
                let data_clone = Arc::clone(&data_clone);
                let handler = thread::spawn(move || {
                    // 执行函数
                    let mut data = data_clone.lock().unwrap();
                    *data += datum as usize;
                });
                datum_spawn.push(handler);
            }
            for handler in datum_spawn {
                handler.join().unwrap();
            }
            if self.threads_data.is_empty() {
                break;
            }
        }
    }
}
fn main() {
    let vec = (0..10000).collect::<Vec<i32>>();
    let mut pool = ThreadPool::new(vec, 10);
    println!("总数是 {:?}", pool.threads.lock().unwrap());
    pool.set_extend_data((91..101).collect::<Vec<i32>>());
    println!("搜搜{:?}", pool.threads.lock().unwrap());
}
