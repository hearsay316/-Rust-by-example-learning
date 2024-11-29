use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::fs;
use tokio::sync::mpsc::{self, Sender};
use tokio::task;

#[derive(Debug)]
struct PathInfo {
    path: String,   // 文件路径
    suffix: String, // 后缀格式
}
impl PathInfo {
    fn new(path: String, suffix: String) -> Self {
        PathInfo { path, suffix }
    }
    fn from_vec(path: Vec<String>, suffix: String) -> Vec<Self> {
        let mut vec = vec![];
        for i in path {
            vec.push(PathInfo::new(i, suffix.clone()));
        }
        vec
    }
}

#[derive(Debug)]
struct Options {
    path_infos: Vec<PathInfo>,
    tx: Option<Sender<(PathBuf, String)>>,
    joins: Option<Vec<task::JoinHandle<()>>>,
}

impl Options {
    async fn new(args: Vec<String>) -> Self {
        let current_dir = std::env::current_dir().expect("获取当前目录失败");
        let current_dir_str = current_dir.to_string_lossy().to_string();
        println!("当前目录 :{:?}", current_dir_str);

        if args.len() < 2 {
            panic!("参数不正确");
        }

        let mut args = args.clone();
        let mut path = args.drain(1..).collect::<Vec<String>>();
        let suffix = path
            .iter()
            .find(|s| !s.contains(".") && !s.contains("/"))
            .cloned()
            .unwrap_or("mp4".to_string());

        let (tx, mut rx) = mpsc::channel(100);
        let rx = Arc::new(Mutex::new(rx));
        let joins = task_spawn(rx).await;
        let path_infos = PathInfo::from_vec(path, suffix);

        Options {
            path_infos,
            tx: Some(tx),
            joins: Some(joins),
        }
    }

    async fn update_name_suffix(mut self) {
        if let Some(tx) = self.tx.clone() {
            let mut handles = vec![];
            for path_info in self.path_infos.drain(..) {
                let tx = tx.clone();
                handles.push(task::spawn(async move {
                    if get_path_file(&path_info.path).await {
                        tx.send((PathBuf::from(&path_info.path), path_info.suffix.to_string()))
                            .await
                            .expect("发送消息失败");
                    } else {
                        update_path_name_suffix(&path_info, tx.clone()).await;
                    }
                }));
            }
            for handle in handles {
                handle.await.expect("任务失败");
            }
        }
        println!("修改完成");
    }
}

async fn update_path_name_suffix(data: &PathInfo, tx: Sender<(PathBuf, String)>) {
    let mut entries = fs::read_dir(&data.path).await.expect("读取文件夹失败");
    for entry in entries.next_entry().await.expect("cyow") {
        let file_path = entry.path();
        if file_path.is_file() {
            tx.clone()
                .send((file_path, data.suffix.to_string()))
                .await
                .expect("发送消息失败");
        } else {
            let file_path = file_path.to_string_lossy().to_string();
            let path_info = PathInfo::new(file_path, data.suffix.to_string());
            Box::pin(update_path_name_suffix(&path_info, tx.clone())).await;
        }
    }
}

async fn get_path_file(path: &str) -> bool {
    let metadata = fs::metadata(path).await.expect("路径不存在");
    metadata.is_file()
}

async fn update_file_name_suffix(entry: PathBuf, suffix: String) {
    let mut ren_name = entry.clone();
    let old_name = entry.clone();
    ren_name.set_extension(&suffix);
    println!("{:?}", ren_name);
    println!("正在修改文件名称,{:?},{:?},{}", ren_name, old_name, suffix);
    fs::rename(old_name, ren_name).await.expect("改名字失败");
    println!("修改 {:?} 成功", entry);
}

#[tokio::main]
async fn main() {
    let path_dir = "./test1";
    let path_file = "1.mp3";
    let path = format!("{path_dir}/{path_file}");
    let options = Options::new(vec!["".to_string(), path]).await;
    options.update_name_suffix().await;
}
