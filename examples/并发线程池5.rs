use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;
#[derive(Debug)]
pub struct Data {
    pub path: String,
    pub fix: String,
}

impl Data {
    pub fn new(vec: Vec<String>) -> Self {
        let current_dir = env::current_dir().expect("无法获取当前目录");
        let pash_str = current_dir.to_string_lossy().to_string();
        let path = vec.get(2).unwrap_or_else(|| &pash_str).clone();
        let fix = vec.get(1).unwrap_or(&"mp4".to_string()).clone();
        Self { path, fix }
    }
    pub fn from(vec: Vec<String>) -> Vec<Self> {
        let current_dir = env::current_dir().expect("无法获取当前目录");
        let pash_str = current_dir.to_string_lossy().to_string();
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

// impl Drop for DataChange {
//     fn drop(&mut self) {
//         println!("{:?}",self);
//         let tx_clone =  self.tx.take();
//         drop(tx_clone.unwrap());
//         for i in self.json_hand.take().unwrap(){
//             i.await.unwrap();
//         }
//     }
// }

#[derive(Debug)]
pub struct DataChange {
    pub data: Vec<Data>,
    pub tx: Option<mpsc::Sender<Vec<String>>>,
    pub json_hand: Option<Vec<JoinHandle<()>>>,
}
impl DataChange {
    pub async fn new(vec: Vec<String>) -> Self {
        println!("95行");
        // let (tx, rx) = mpsc::channel(32);
        // let rx = Arc::new(Mutex::new(vec![]));
        let josn_hand: Option<Vec<JoinHandle<()>>> = None;
        // joson_hande(rx).await;
        let data = Data::from(vec);
        // println!("第96行//{:?}{:?}{:?}", data, None, josn_hand);
        Self {
            data,
            tx: None,
            json_hand: None,
        }
    }
    pub async fn run(mut self) {
        let mut files = vec![];
        let mut paths = vec![];
        let mut vec = Arc::new(Mutex::new(vec![]));
        let (tx, mut rx) = mpsc::channel(128);
        for i in self.data.drain(..) {
            let vec_clone = vec.clone();
            let tx_clone = tx.clone();
            let handle = tokio::spawn(async move {
                path_name_update(&i.path, &i.fix, tx_clone, vec_clone).await;
            });
            files.push(handle);
        }
        for handle in files {
            handle.await.expect("错误了");
        }

        let mut vec_lock = vec.lock().await;
        for i in vec_lock.drain(..) {
            i.await.unwrap();
        }
        drop(tx);

        while let Some(message) = rx.recv().await {
            println!("GOT = {}", message);
            paths.push(message);
        }
        println!("{:?}", paths);
    }
}

pub async fn path_name_update(
    path: &str,
    fix: &str,
    tx: mpsc::Sender<String>,
    vec: Arc<Mutex<Vec<JoinHandle<()>>>>,
) {
    let mut netries = fs::read_dir(&path).await.unwrap();
    println!("第111行{:?}", netries);
    while let Some(entry) = netries.next_entry().await.expect("sss") {
        let path = entry.path();
        let tx = tx.clone();
        let fix = fix.to_string();
        let vec = vec.clone();
        println!("第118行:{:?}", path);
        let handle = process_entry(path, fix.to_string(), tx); //取消了r                                                         x的传参
    }
}
pub fn process_entry(path: PathBuf, fix: String, tx: mpsc::Sender<String>) -> JoinHandle<()> {
    println!("第155行 {:?}", path);
    tokio::spawn(async move {
        if path.is_file() {
            let new_path = path.to_string_lossy().to_string();
            println!("发送数据{:?}", new_path);
            tx.send(new_path).await.expect("发送消息失败");
        } else {
            let mut vec = Arc::new(Mutex::new(vec![])); //新加的
            let _sub_paths = path_name_update(
                &path.to_string_lossy().to_string(),
                &fix,
                tx.clone(),
                vec.clone(),
            )
            .await;
            let mut json_handlers = vec.lock().await;
            for json in json_handlers.drain(..) {
                json.await.unwrap();
            }
        } //新加的
    })
}
pub async fn joson_hande(rx: Arc<Mutex<Vec<JoinHandle<()>>>>) -> Vec<JoinHandle<()>> {
    let mut hanle = vec![];
    let channel_number = 10;
    for i in 0..channel_number {
        let tx_clone = rx.clone();
        println!("第52行");
        let handle = tokio::spawn(async move {
            // println!("第54行");
            // loop {
            //     println!("第54行");
            //     let tx_clone = tx_clone.lock().await;
            //     match tx_clone {
            //         MutexGuard { .. } => {}
            //     }
            // }
        });
        hanle.push(handle);
    }
    hanle
}
#[cfg(test)]
mod tests {
    use crate::DataChange;

    #[tokio::test]
    async fn test_work() {
        let vec = vec!["".to_string(), "mp4".to_string(), "./examples".to_string()];
        let mut data = DataChange::new(vec).await;
        println!("{:?}", data);
        data.run().await;
    }
}
#[tokio::main]
async fn main() {
    let vec = vec!["".to_string(), "mp4".to_string(), "./examples".to_string()];
    let mut data = DataChange::new(vec).await;
    println!("{:?}", data);
    data.run().await;
}
