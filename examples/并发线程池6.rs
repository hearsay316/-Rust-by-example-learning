// src/main.rs

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4, |s: String| {
        println!("Pool spawned{:?}", s);
    });

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute::<String>("这个是粗俄欧".to_string());
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
// src/lib.rs
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool<T>
where
    T: Send + 'static,
{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<T>>,
}

impl<T> ThreadPool<T>
where
    T: Send + 'static,
{
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize, f: impl FnOnce(T) + Send + 'static + Clone) -> ThreadPool<T> {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let job = f.clone();
            workers.push(Worker::new(id, Arc::clone(&receiver), job));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: T) {
        self.sender.clone().unwrap().send(f).unwrap();
    }
}

impl<T> Drop for ThreadPool<T>
where
    T: Send + 'static,
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

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new<T: Send + 'static>(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<T>>>,
        f: impl FnOnce(T) + Send + 'static + Clone,
    ) -> Worker {
        let f = f.clone();
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    let f = f.clone();
                    println!("Worker {id} got a job; executing.");
                    f(job);
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
