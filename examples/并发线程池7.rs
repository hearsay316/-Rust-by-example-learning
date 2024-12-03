use std::path::PathBuf;
use std::sync::Arc;
use std::{env, fs};
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;
#[derive(Debug)]
pub struct Data {
    pub path: String,
    pub fix: String,
}
impl Data {
    pub fn from(vec: Vec<String>) -> Vec<Self> {
        let current_dir = env::current_dir().expect("无法获取当前目录");
        let path = vec[2..].to_vec();
        let fix = vec[1].to_string();
        let mut add = vec![];
        for i in path {
            add.push(Data {
                path: i,
                fix: fix.clone(),
            });
        }
        add
    }
}
#[derive(Debug)]
pub struct ThreadPool {
    pub tx: Option<mpsc::Sender<(String, String)>>,
    handles: Vec<JoinHandle<()>>,
}
impl ThreadPool {
    pub fn new(
        channel_number: usize,
        task_fn: Arc<dyn Fn(String, String) + Send + Sync + 'static>,
    ) -> Self {
        let (tx, rx) = mpsc::channel(channel_number);
        let rx = Arc::new(Mutex::new(rx));
        let mut handles = vec![];
        for _ in 0..channel_number {
            let rx_clone = rx.clone();
            let task_fn_clone = task_fn.clone();
            let handle = tokio::spawn(async move {
                loop {
                    let task_fn_clone = task_fn_clone.clone();
                    let task = rx_clone.lock().await.recv().await;
                    match task {
                        Some((path, fix)) => {
                            println!("第49行{:?}发送成功", path);
                            task_fn_clone(path, fix);
                        }
                        None => {
                            break;
                        }
                    }
                }
            });
            handles.push(handle);
        }
        Self {
            tx: Some(tx),
            handles,
        }
    }
    pub async fn submit(&self, path: String, fix: String) {
        if let Some(tx) = &self.tx {
            tx.send((path, fix)).await.unwrap();
        }
    }
    pub async fn join_all(&mut self) {
        self.tx = None;
        for handle in self.handles.drain(..) {
            handle.await.unwrap();
        }
    }
}
#[derive(Debug)]
pub struct DataChange {
    pub data: Vec<Data>,
    pub thread_pool: ThreadPool,
}

impl DataChange {
    pub async fn new(
        vec: Vec<String>,
        task_fn: Option<Arc<dyn Fn(String, String) + Send + Sync + 'static>>,
        chaanel_number: Option<usize>,
    ) -> Self {
        pub fn process_file(path: String, fix: String) {
            let a = PathBuf::from(&path);
            let old = a.clone();
            let mut new = a.clone();
            new.set_extension(&fix);
            if let Err(e) = fs::rename(old, new) {
                println!("{:?} 失败 {}", a, e);
            } else {
                println!("{:?} 成功", a);
            }
        }
        let fnn = Arc::new(process_file);
        let task_fn = task_fn.unwrap_or(fnn.clone());
        let chaanel_number = chaanel_number.unwrap_or(10);
        let thread_pool = ThreadPool::new(chaanel_number, task_fn);
        let data = Data::from(vec);
        Self { data, thread_pool }
    }
    pub async fn run(&mut self) {
        let mut files = vec![];
        let mut paths = vec![];
        let (tx, mut rx) = mpsc::channel(32);
        for i in self.data.drain(..) {
            let tx_clone = tx.clone();
            let handle = tokio::spawn(async move {
                path_name_update(&i.path, &i.fix, tx_clone).await;
            });
            files.push(handle);
        }
        for handle in files {
            handle.await.unwrap();
        }
        drop(tx);
        while let Some(i) = rx.recv().await {
            println!("第78行//{:?}", i);
            paths.push(i);
        }
        for path in paths {
            self.thread_pool.submit(path.0, path.1).await;
        }
        self.thread_pool.join_all().await;
    }
}
pub async fn path_name_update(path: &str, fix: &str, tx: Sender<(String, String)>) {
    let mut netries = tokio::fs::read_dir(&path).await.unwrap();
    while let Some(entry) = netries.next_entry().await.unwrap() {
        let path = entry.path();
        let tx = tx.clone();
        let fix = fix.to_string();
        println!("第145行");
        process_entry(path, fix.to_string(), tx);
    }
    pub fn process_entry(
        path: PathBuf,
        fix: String,
        tx: mpsc::Sender<(String, String)>,
    ) -> JoinHandle<()> {
        println!("第158行");
        tokio::spawn(async move {
            if path.is_file() {
                let new_path = path.to_string_lossy().to_string();
                tx.send((new_path, fix)).await.expect("发送消息失败");
            } else {
                let _sub_paths =
                    path_name_update(&path.to_string_lossy().to_string(), &fix, tx.clone()).await;
            }
        })
    }
}

#[tokio::main]
async fn main() {
    let vec = vec!["".to_string(), "mp4".to_string(), "./test".to_string()];
    let mut data = DataChange::new(vec, None, Some(100)).await;
    println!("{:?}", data);
    data.run().await;
}
