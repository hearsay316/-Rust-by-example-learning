use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;
use std::{env, fs, thread};

pub struct Touch {
    path: String,
    fix: String,
}

impl Touch {
    pub fn new(vec: Vec<String>) -> Self {
        let current_dir = env::current_dir().expect("无法获取当前目录");
        let push_str = current_dir.to_string_lossy().to_string();
        let path = vec.get(1).unwrap_or_else(|| &push_str).clone();
        println!("第15行{:?}", path);
        let fix = vec.get(2).unwrap_or(&"mp4".to_string()).clone();
        Self { path, fix }
    }

    pub fn get_path(&self) {
        println!("第19行{}", self.path);
        let x = path_is_files(&self.path);
        println!("第21行{}", x);
        if x {
            let path = PathBuf::from(&self.path);
            path_change_dir(path, &self.fix);
        } else {
            path_name_update(&self.path, &self.fix);
        }
    }
}

fn count_file_number(path: &str, fix: &str) {
    let mut vec = vec![];
    let back_value = path_name_update_v2(path, fix, &mut vec);
    // for i in vec{
    //     println!("{:?}", i);
    // }
    println!("{:?}", back_value);
    println!("{}", back_value.len());
    let length = back_value.len() as f64;
    for (index, _value) in back_value.iter().enumerate() {
        println!(
            "当前索引: {}%",
            ((index + 1) as f64 / length * 100f64) as i32
        );
    }
}

fn path_name_update_v2<'a>(path: &str, fix: &str, vec: &'a mut Vec<String>) -> &'a Vec<String> {
    let entries = fs::read_dir(&path).unwrap();
    println!("{:?}1111111", entries);
    for i in entries {
        if let Ok(i) = i {
            let path = i.path();
            if path.is_file() {
                vec.push(path.to_string_lossy().to_string());
            } else {
                path_name_update_v2(&path.to_string_lossy().to_string(), fix, vec);
            }
        }
    }
    vec
}

fn path_name_update_v4(path: &str, fix: &str) -> Vec<String> {
    let (tx, rx) = mpsc::channel();
    let mut vec = Arc::new(Mutex::new(vec![]));
    println!("第121行/");
    let _vec_json = path_name_update_v3(path, fix, tx.clone(), vec.clone());
    let mut all_files = vec![];
    let mut json_handlers = vec.lock().unwrap();
    for json in json_handlers.drain(..) {
        json.join().unwrap();
    }
    drop(tx);
    for received in rx.iter() {
        all_files.push(received);
    }

    all_files
}

fn path_name_update_v3(
    path: &str,
    fix: &str,
    tx: mpsc::Sender<String>,
    vec: Arc<Mutex<Vec<JoinHandle<()>>>>,
) {
    let entries = fs::read_dir(&path).unwrap();

    for i in entries {
        if let Ok(entry) = i {
            let path = entry.path();
            let tx = tx.clone();
            let fix = fix.to_string();
            println!("第145行");
            let handle = process_entry(path, fix.to_string(), tx);
            //  handle.join().unwrap();
            let mut vec_lock = vec.lock().unwrap();
            vec_lock.push(handle);
        }
    }
}

fn process_entry(path: PathBuf, fix: String, tx: mpsc::Sender<String>) -> JoinHandle<()> {
    println!("第155行");
    thread::spawn(move || {
        if path.is_file() {
            let new_path = path.to_string_lossy().to_string();
            tx.send(new_path).unwrap();
        } else {
            let mut vec = Arc::new(Mutex::new(vec![]));
            let _sub_paths = path_name_update_v3(
                &path.to_string_lossy().to_string(),
                &fix,
                tx.clone(),
                vec.clone(),
            );
            let mut json_handlers = vec.lock().unwrap();

            for json in json_handlers.drain(..) {
                json.join().unwrap();
            }
        }
    })
}

fn path_name_update(path: &str, fix: &str) {
    let entries = fs::read_dir(&path).unwrap();
    for i in entries {
        if let Ok(i) = i {
            let path = i.path();
            if path.is_file() {
                path_change_dir(path, fix);
            } else {
                path_name_update(&path.to_string_lossy().to_string(), fix);
            }
        }
    }
}

//
fn path_is_files(path: &str) -> bool {
    let path = Path::new(path);
    let metadata = fs::metadata(path).expect("无法判断文件类型");
    metadata.is_file()
}

fn path_change_dir(path: PathBuf, fix: &str) {
    let old = path.clone();
    let mut new = path.clone();
    new.set_extension(fix);
    println!("{:?}:::  {:?}////", new, old);
    if let Err(e) = fs::rename(old, new) {
        println!("{:?}失败 {}", path, e);
    } else {
        println!("{:?}成功 ", path);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;
    use std::{collections::HashSet, fs};

    #[derive(Debug)]
    struct FileManager {
        files: HashSet<String>,
        delete_files: HashSet<PathBuf>,
        paths: HashSet<String>,
        delete_paths: HashSet<String>,
    }

    impl FileManager {
        fn new(vec: &Vec<&str>, lec: &str) -> Self {
            // 1.文件数据 2.文件夹数据 3.改后缀之后的数据
            // 1.传入的数据 2.后缀名
            //22
            let mut files = HashSet::new();
            let mut paths = HashSet::new();
            let mut delete_files = HashSet::new();
            let mut delete_paths = HashSet::new();
            for str in vec {
                if str.len() < 1 {
                    continue;
                };
                if !str.contains("/") {
                    continue;
                };
                let str_array: Vec<&str> = str.split("/").collect();
                let str_array_son1 = str_array[0];
                let str_array_son2 = str_array[0..str_array.len() - 1].join("/");
                println!("{:?},3111111", str_array);
                println!("{}", str_array.len());
                println!("{:?}str_array_son1", str_array_son1);
                println!("{:?}str_array_son2", str_array_son2);
                if str_array_son1 == "." && str_array.len() > 2 {
                    println!("{}!!!!!!!!!!!!!!", str_array[0..2].join("/"));
                    paths.insert(str_array[0..str_array.len() - 1].join("/"));
                    delete_paths.insert(str_array[0..2].join("/"));
                    files.insert(str.to_string().clone());
                    delete_files.insert(change_file_name(str.to_string(), lec));
                    continue;
                };
                if str_array_son2 == "." && str_array.len() == 2 {
                    files.insert(str.to_string());
                    println!("{:?}", str.to_string());
                    println!("{:?}", files);
                    paths.insert(format!("{}/", str_array_son1));
                    delete_paths.insert(format!("{}/", str_array_son1));
                    delete_files.insert(change_file_name(str.to_string(), lec));
                    continue;
                }
                if str_array_son2 != "." && str_array.len() == 2 {
                    files.insert(format!("./{}", str.to_string()));
                    delete_files.insert(change_file_name(format!("./{}", str.to_string()), lec));
                    continue;
                }
                // return "".to_string();
                paths.insert(format!("./{}", str_array_son1));
                // delete_paths.insert(format!("./{}", str_array_son1));
                files.insert(format!("./{}", str));
                delete_files.insert(change_file_name(format!(".{}", str), lec));
            }
            FileManager {
                files,
                delete_paths,
                paths,
                delete_files,
            }
        }

        fn action_create(&self) {
            for i in &self.paths {
                match fs::create_dir_all(i) {
                    Ok(_) => {}
                    Err(e) => println!("{:?}", e),
                }
            }
            for i in &self.files {
                println!("{:?}", i);
                match fs::File::create(i) {
                    Ok(_) => {}
                    Err(e) => println!("{:?}", e),
                }
            }
        }

        fn action_delete(&self) {
            for i in &self.files {
                match fs::remove_file(i) {
                    Ok(_) => {}
                    Err(e) => println!("{:?}", e),
                };
            }
            for i in &self.delete_paths {
                match fs::remove_dir_all(i) {
                    Ok(_) => {}
                    Err(e) => println!("{:?}", e),
                }
            }
        }

        fn action_delete_change_name(&self) {
            for i in &self.delete_files {
                println!("{:?}", i);
                match fs::remove_file(i) {
                    Ok(_) => {}
                    Err(e) => println!("{:?}", e),
                }
            }
            for i in &self.delete_paths {
                match fs::remove_dir_all(i) {
                    Ok(_) => {}
                    Err(e) => println!("{:?}", e),
                }
            }
        }
    }

    fn change_file_name(str: String, houri: &str) -> PathBuf {
        let mut path = PathBuf::from(str);
        path.set_extension(houri);
        path
    }

    #[test]
    fn path_name_update_v2_test() {
        count_file_number("./examples", "mp4");
        println!()
    }

    #[test]
    fn path_name_update_v4_test() {
        println!("打印");
        let path = path_name_update_v4("./examples", "mp4");
        println!("{:?}", path.len());
    }
}

//     #[test]
//     fn working_path_change_dir() {
//         // 测试成功
//         let vec = &vec!["./ss.sa"];
//         let doc = "mp4";
//         let file_manager = FileManager::new(vec, doc);
//         println!("{:?}", file_manager);
//         file_manager.action_create();
//         let path1 = PathBuf::from(vec[0]);
//         path_change_dir(path1, doc);
//         let res1 = path_is_files("./ss.mp4");
//         assert_eq!(res1, true);
//         file_manager.action_delete_change_name();
//         //测试文件夹
//         let vec = &vec!["./ss"];
//         let doc = "mp4";
//         let file_manager = FileManager::new(vec, doc);
//         file_manager.action_create();
//         let path1 = PathBuf::from(vec[0]);
//         path_change_dir(path1, doc);
//         let res1 = path_is_files("./ss.mp4");
//         assert_eq!(res1, true);
//         file_manager.action_delete_change_name();
//     }

//     #[test]
//     fn working_touch_new() {
//         // 测试正确
//         let args = vec![String::from("test"), String::from("/path/to/dir"), String::from("mp4")];
//         let l = super::touch::new(args);
//         assert_eq!(l.path, "/path/to/dir");
//         assert_eq!(l.fix, "mp4");
//         // 测试失败
//         // let ss = vec![String::from("test") ];
//         // let l = super::touch::new(ss);
//         // assert_eq!(l.path,"/cloudide/workspace/day-8");
//         // assert_eq!(l.fix, "mp4");
//     }

//     #[test]
//     fn working_path_is_files() {
//         //测试目录
//         let vec = &vec!["./ss"];
//         let doc = "mp4";
//         let file_manager = FileManager::new(vec, doc);
//         println!("{:?}", file_manager);
//         file_manager.action_create();
//         let ret1 = path_is_files(vec[0]);
//         assert_eq!(ret1, true);
//         file_manager.action_delete();
//         //测试文件
//         let vec2 = &vec!["./ss/ss.sa"];
//         let doc = "mp4";
//         let file_manager = FileManager::new(vec2, doc);
//         println!("{:?}", file_manager);
//         file_manager.action_create();
//         let ret2 = path_is_files(vec2[0]);
//         assert_eq!(ret2, true);
//         file_manager.action_delete();
//         //测试失败
//         // let ret3 = path_is_files("ss.sa");
//         // assert_eq!(ret3, false);
//     }

//     #[test]
//     fn working_path_name_update() {
//         //测试正确
//         let vec = &vec!["./ss", "./ss/ss.sa"];
//         let doc = "mp4";
//         let file_manager = FileManager::new(vec, doc);
//         println!("{:?}", file_manager);
//         file_manager.action_create();
//         path_name_update(vec[0], doc);
//         let res1 = path_is_files("./ss/ss.mp4");
//         assert_eq!(res1, true);
//         file_manager.action_delete();
//         file_manager.action_delete_change_name();
//         //测试失败
//         // let vec2 = &vec!["./ss"];
//         // let doc = "mp4";
//         // let file_manager = FileManager::new(vec2, doc);
//         // println!("{:?}", file_manager);
//         // file_manager.action_create();
//         // path_name_update(vec2[0], doc);
//         // let res2 = path_is_files("./ss.mp4");
//         // assert_eq!(res2, true);
//         // file_manager.action_delete();
//     }

//     #[test]
//     fn working_get_path() {
//         // 测试文件
//         let args = vec![String::from("test"), String::from("testing.txt"), String::from("mp4")];
//         let vec = &vec!["./testing.txt"];
//         let doc = "mp4";
//         let file_manager = FileManager::new(vec, doc);
//         println!("{:?}", file_manager);
//         file_manager.action_create();
//         let res1 = Touch::new(args);
//         println!("{:?}", res1);
//         res1.get_path();
//         let res2 = path_is_files("./testing.mp4");
//         assert_eq!(res2, true);
//         file_manager.action_delete_change_name();
//         //测试文件夹
//         let args = vec![String::from("./test"), String::from("./test/test.re"), String::from("mp4")];
//         let vec = &vec!["./test", "./test/test.re"];
//         let doc = "mp4";
//         let file_manager = FileManager::new(vec, doc);
//         println!("{:?}", file_manager);
//         file_manager.action_create();
//         let res1 = Touch::new(args);
//         res1.get_path();
//         let res2 = path_is_files("./test/test.mp4");
//         assert_eq!(res2, true);
//         file_manager.action_delete_change_name();
//     }
// }

fn main() {}
