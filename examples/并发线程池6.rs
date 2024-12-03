// src/main.rs

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool<T>
where
    T: Send + 'static + std::fmt::Debug,
{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<T>>,
}

impl<T> ThreadPool<T>
where
    T: Send + 'static + std::fmt::Debug,
{
    pub fn new(size: usize, f: Option<Arc<dyn Fn(T) + Send + Sync + 'static>>) -> ThreadPool<T> {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        let default_fn = Arc::new(|msg: T| {
            println!("Default handler: {:?}", msg);
        });

        let handler = f.unwrap_or(default_fn);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), Arc::clone(&handler)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute(&self, msg: T) {
        self.sender.as_ref().unwrap().send(msg).unwrap();
    }
}

impl<T> Drop for ThreadPool<T>
where
    T: Send + 'static + std::fmt::Debug,
{
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new<T>(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<T>>>,
        f: Arc<dyn Fn(T) + Send + Sync + 'static>,
    ) -> Worker
    where
        T: Send + 'static + std::fmt::Debug,
    {
        let thread = thread::spawn(move || loop {
            println!("Worker {id}开启线程");
            match receiver.lock().unwrap().recv() {
                Ok(msg) => {
                    println!("Worker {id} got a job; executing.");
                    f(msg);
                    thread::sleep(Duration::from_secs(5));
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 使用默认处理函数
    let pool: ThreadPool<String> = ThreadPool::new(4, None);

    // 或者提供自定义处理函数
    // let custom_handler = Arc::new(|msg: String| {
    //     println!("Custom handler: {}", msg);
    // });
    // let pool: ThreadPool<String> = ThreadPool::new(4, Some(custom_handler));

    for stream in listener.incoming().take(20) {
        let _stream = stream.unwrap();
        pool.execute("这个是粗俄欧".to_string());
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
