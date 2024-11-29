use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;
use std::{env, fs, thread};

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
    vec_joins: Option<Vec<JoinHandle<()>>>,
}
impl Drop for Options {
    fn drop(&mut self) {
        println!("线程结束");
        drop(self.tx.take().unwrap());
        let handler = self.vec_joins.take().unwrap();
        for handle in handler {
            handle.join().expect("线程失败");
        }
    }
}
fn thread_spawn(rx: Arc<Mutex<Receiver<(PathBuf, String)>>>) -> Vec<JoinHandle<()>> {
    let mut handles = vec![];
    for i in 0..4 {
        println!("正再循行");
        let rx = rx.clone();
        let handle = thread::spawn(move || loop {
            let data = rx.lock().unwrap().recv();
            match data {
                Ok((path, suffix)) => {
                    update_file_name_suffix(path.clone(), suffix);
                    println!("修改 {:?} 成功", path);
                }
                Err(_) => {
                    println!(" 线程失败{}", i);
                    break;
                }
            }
        });
        handles.push(handle);
    }
    handles
}
impl Options {
    fn new(args: Vec<String>) -> Self {
        // 获取当前工作目录
        let current_dir = env::current_dir().expect("获取当前目录失败");
        // 将 PathBuf 转换为字符串
        let current_dir_str = current_dir.to_string_lossy().to_string();
        println!("当前目录 :{:?}", current_dir_str);
        let mut args: Vec<String> = args.clone();
        let path = args.drain(1..).collect::<Vec<String>>();
        let suffix = "mp4".to_string();
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let vec_joins = thread_spawn(rx);
        let path_infos = PathInfo::from_vec(path, suffix);
        Options {
            path_infos,
            tx: Some(tx),
            vec_joins: Some(vec_joins),
        }
    }
    // 修改文件夹后缀
    fn update_name_suffix(mut self) {
        let tx = self.tx.clone().unwrap();
        let mut handle = vec![];
        for path_info in self.path_infos.drain(..) {
            let tx = tx.clone();
            handle.push(thread::spawn(move || {
                if get_path_file(&path_info.path) {
                    tx.send((PathBuf::from(&path_info.path), path_info.suffix.to_string()))
                        .expect("发送消息失败");
                    // update_file_name_suffix(PathBuf::from(&self.path_info.path), &self.path_info.suffix);
                } else {
                    update_path_name_suffix(&path_info, tx.clone());
                }
            }));
        }
        for i in handle {
            i.join().expect("线程失败");
        }
        println!("修改完成");
    }
    // fn end(self) {
    //     let Options { tx, vec_joins, .. } = self;
    //     drop(tx.unwrap());
    //     let handler = vec_joins.unwrap();
    //     for handle in handler {
    //         handle.join().expect("线程失败");
    //     }
    // }
}
fn update_path_name_suffix(data: &PathInfo, tx: Sender<(PathBuf, String)>) {
    let entries = fs::read_dir(&data.path).expect("读取文件夹失败");
    for entry in entries {
        if let Ok(entry) = entry {
            let file_path = entry.path();
            if file_path.is_file() {
                tx.clone()
                    .send((file_path, data.suffix.to_string()))
                    .expect("发送消息失败");
            } else {
                let file_path = file_path.to_string_lossy().to_string();
                let path_info = PathInfo::new(file_path, data.suffix.to_string());
                update_path_name_suffix(&path_info, tx.clone());
            }
        }
    }
}

fn get_path_file(path: &str) -> bool {
    let metadata = fs::metadata(path).expect("路径不存在");
    metadata.is_file()
}
fn update_file_name_suffix(entry: PathBuf, suffix: String) {
    let mut ren_name = entry.clone();
    let old_name = entry.clone();
    ren_name.set_extension(&suffix);
    println!("{:?}", ren_name);
    println!("正在修改文件名称,{:?},{:?},{}", ren_name, old_name, suffix);
    fs::rename(old_name, ren_name).expect("改名字失败");
    println!("修改 {:?} 成功", entry);
}
fn main() {
    // let args: Vec<String> = env::args().collect();
    // let options = Options::new(args);
    // println!("数据是: {:?}", options);
    // options.update_name_suffix();

    let path_dir = "./test1";
    let path_file = "1.mp3";
    let path = format!("{path_dir}/{path_file}");
    // create_dir_files_v2(vec![&path]);
    // assert_eq!(get_path_file(&path), true);
    let mut options = Options::new(vec!["".to_string(), path]);
    options.update_name_suffix();
}
#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_get_path_file() {
        let path_dir = "./test1";
        let path_file = "1.mp3";
        let path = format!("{path_dir}/{path_file}");
        let path2 = "./test2/2.mp3".to_string();
        create_dir_files_v2(vec![&path, &path2]);
        assert_eq!(get_path_file(&path), true);
        let mut options = Options::new(vec!["".to_string(), path, path2]);
        println!("{:?}", options);
        options.update_name_suffix();
        // options.end();
        delete_dir(path_dir);
        delete_dir("./test2");
    }
    #[test]
    fn test_update_file_name_suffix() {
        let path_dir = "./test2";
        let path_file = "1";
        let old_suffix = "mp3";
        let new_suffix = "mp4";
        let path = format!("{path_dir}/{path_file}.{old_suffix}");
        let new_path = format!("{path_dir}/{path_file}.{new_suffix}");
        create_dir_files(path_dir, vec![&format!("{path_file}.{old_suffix}")]);
        update_file_name_suffix(PathBuf::from(path), new_suffix.to_string());
        assert_eq!(get_path_file(&new_path), true);
        delete_dir(path_dir);
    }
    #[test]
    fn test_options_new() {
        let args: Vec<String> = vec![
            "".to_string(),
            "D:\\video\\2023-05-01\\1.mp4".to_string(),
            "mp4".to_string(),
        ];
        let options = Options::new(args);
        println!("数据是: {:?}", options);
        assert_eq!(
            options.path_infos[0].path,
            "D:\\video\\2023-05-01\\1.mp4".to_string()
        );
        assert_eq!(options.path_infos[0].suffix, "mp4".to_string());
    }

    // #[test]
    // fn test_update_path_name_suffix() {
    //     let path_dir = "./test3";
    //     let path_file = "1";
    //     let old_suffix = "mp3";
    //     let new_suffix = "mp4";
    //     let new_path = format!("{path_dir}/{path_file}.{new_suffix}");
    //     create_dir_files(path_dir, vec![&format!("{path_file}.{old_suffix}")]);
    //     let path_info = PathInfo::new(path_dir.to_string(), new_suffix.to_string());
    //     update_path_name_suffix(&path_info);
    //     assert_eq!(get_path_file(&new_path), true);
    //     delete_dir(path_dir);
    // }

    #[test]
    fn test_update_name_suffix() {
        let path_dir = "./test4";
        let path_file = "1";
        let old_suffix = "mp3";
        let new_suffix = "mp4";
        let path = format!("{path_dir}/{path_file}.{old_suffix}");
        let new_path = format!("{path_dir}/{path_file}.{new_suffix}");
        let args: Vec<String> = vec!["".to_string(), path.clone(), new_suffix.to_string()];
        create_dir_files(path_dir, vec![&format!("{path_file}.{old_suffix}")]);
        let options = Options::new(args);
        println!("数据是: {:?}", options);
        options.update_name_suffix();
        assert_eq!(get_path_file(&new_path), true);
        delete_dir(path_dir);
    }
    fn create_dir_files(path_dir: &str, file_vec: Vec<&str>) {
        let dir_path = path_dir;
        // 尝试创建目录
        match fs::create_dir_all(dir_path) {
            Ok(_) => println!("目录创建成功或已存在"),
            Err(e) => println!("创建目录时发生错误: {:?}", e),
        }
        if file_vec.is_empty() {
            return;
        }
        for file_name in file_vec {
            let file_path = format!("{}/{}", dir_path, file_name);
            match fs::File::create(&file_path) {
                Ok(_) => println!("文件创建成功"),
                Err(e) => println!("创建文件时发生错误: {:?}", e),
            }
        }
    }

    fn create_dir_files_v2(file_vec: Vec<&str>) {
        for file in file_vec {
            let arr = file.split("/").collect::<Vec<_>>();
            let dir_path = arr[0..arr.len() - 1].join("/");
            match fs::create_dir_all(&dir_path) {
                Ok(_) => println!("目录创建成功或已存在"),
                Err(e) => println!("创建目录时发生错误: {:?}", e),
            }
            match fs::File::create(&file) {
                Ok(_) => println!("文件创建成功"),
                Err(e) => println!("创建文件时发生错误: {:?}", e),
            }
        }
    }
    fn delete_dir(path: &str) {
        match fs::remove_dir_all(path) {
            Ok(_) => println!("删除目录成功"),
            Err(e) => println!("删除目录时发生错误: {:?}", e),
        }
    }
    #[test]
    fn test_delete_dir_v2() {
        delete_dir_v2(&vec!["./test/1.txt", "/test/2.txt", "./sss.ts"]);
    }
    fn delete_dir_v2(vec: &Vec<&str>) {
        vec.iter()
            .filter_map(|path| {
                if !path.contains("/") {
                    return None;
                }
                let dir_path = path.split("/").collect::<Vec<_>>();
                if dir_path[0] == "." && dir_path.len() == 2 {
                    return Some(("".to_string(), path.to_string()));
                }
                if dir_path[0] == "." && dir_path.len() > 2 {
                    return Some((dir_path[0..2].join("/"), "".to_string()));
                };
                return Some((format!(".{}", dir_path[0..2].join("/")), "".to_string()));
            })
            .for_each(|(dir, path)| {
                if !dir.is_empty() {
                    delete_dir(&dir);
                }
                if !path.is_empty() {
                    match fs::remove_file(path) {
                        Ok(_) => println!("删除文件成功"),
                        Err(e) => println!("删除文件时发生错误: {:?}", e),
                    }
                }
            })
    }
    #[test]
    fn test_delete_dir_v3() {
        let dir_data = DirData::new(
            &vec!["./ss/test/1.txt", "test/2.txt", "./sss.ts", "zz.ds"],
            "mp4",
        );
        dir_data.create_dir_data();
        dir_data.delete_dir_data();
        println!("{:?}", dir_data);
    }
    fn change_extension(path: &str, new_ext: &str) -> PathBuf {
        let mut path_buf = PathBuf::from(path);
        path_buf.set_extension(new_ext);
        path_buf
    }
    #[derive(Debug)]
    struct DirData {
        dir_path: HashSet<String>,
        dir_path_all: HashSet<String>,
        file_path: HashSet<String>,
        new_file_path: HashSet<PathBuf>,
    }
    impl DirData {
        fn new(vec: &Vec<&str>, fix: &str) -> Self {
            let mut dir_path = HashSet::new();
            let mut dir_path_all = HashSet::new();
            let mut file_path = HashSet::new();
            let mut new_file_path = HashSet::new();
            for path in vec {
                if !path.contains("/") {
                    file_path.insert(format!("./{}", path));
                    new_file_path.insert(change_extension(&format!("./{}", path), fix));
                    continue;
                }
                let dir_path_arr = path.split("/").collect::<Vec<_>>();
                println!("{:?}", dir_path_arr);
                if dir_path_arr[0] == "." && dir_path_arr.len() == 2 {
                    file_path.insert(path.to_string());
                    new_file_path.insert(change_extension(path, fix));
                    continue;
                }
                if dir_path_arr[0] == "." && dir_path_arr.len() > 2 {
                    dir_path.insert(dir_path_arr[0..dir_path_arr.len() - 1].join("/"));
                    dir_path_all.insert(dir_path_arr[0..dir_path_arr.len() - 1].join("/"));
                    file_path.insert(path.to_string());
                    new_file_path.insert(change_extension(path, fix));
                    continue;
                };
                let mut add_fix = ".";
                if dir_path_arr[1] != "/" {
                    add_fix = "./";
                }
                dir_path.insert(format!(
                    "{}{}",
                    add_fix,
                    dir_path_arr[0..dir_path_arr.len() - 1].join("/")
                ));
                dir_path_all.insert(format!(
                    "{}{}",
                    add_fix,
                    dir_path_arr[0..dir_path_arr.len() - 1].join("/")
                ));
                file_path.insert(format!("{}{}", add_fix, path));
                new_file_path.insert(change_extension(&format!("{}{}", add_fix, path), fix));
            }
            Self {
                dir_path,
                dir_path_all,
                file_path,
                new_file_path,
            }
        }
        fn create_dir_data(&self) {
            self.dir_path_all
                .iter()
                .for_each(|dir| match fs::create_dir_all(dir) {
                    Ok(_) => println!("目录创建成功或已存在"),
                    Err(e) => println!("创建目录时发生错误: {:?}", e),
                });
            self.file_path
                .iter()
                .for_each(|file| match fs::File::create(file) {
                    Ok(_) => println!("文件创建成功"),
                    Err(e) => println!("创建文件时发生错误: {:?}", e),
                });
        }
        fn delete_dir_data(&self) {
            self.file_path
                .iter()
                .for_each(|file_path| match fs::remove_file(file_path) {
                    Ok(_) => println!("删除文件成功"),
                    Err(e) => println!("删除文件时发生错误: {:?}", e),
                });
            self.delete_dir_path();
        }
        fn delete_dir_path(&self) {
            self.dir_path
                .iter()
                .for_each(|path| match fs::remove_dir_all(path) {
                    Ok(_) => println!("删除目录成功"),
                    Err(e) => println!("删除目录时发生错误: {:?}", e),
                });
        }
        fn delete_dir_new_data(&self) {
            self.new_file_path
                .iter()
                .for_each(|file_path| match fs::remove_file(file_path) {
                    Ok(_) => println!("删除文件成功"),
                    Err(e) => println!("删除文件时发生错误: {:?}", e),
                });
            self.delete_dir_path();
        }
    }
}
