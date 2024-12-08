use std::{env, fs};

struct FileNum();
impl FileNum {
    fn from(path: Vec<String>) -> i64 {
        if path.len() < 2 {
            panic!("参数错误");
        };
        let path = path[1].clone();
        update_path_name_suffix(&path, 0)
    }
}
fn update_path_name_suffix(path: &str, num: i64) -> i64 {
    let mut num = num;
    let entries = fs::read_dir(path).expect("读取文件夹失败");
    for entry in entries {
        if let Ok(entry) = entry {
            let file_path = entry.path();
            if file_path.is_file() {
                num += 1;
            } else {
                let file_path = file_path.to_string_lossy().to_string();
                let sum_num = update_path_name_suffix(&file_path, 0);
                num += sum_num;
            }
        }
    }
    num
}
fn main() {
    let vec = env::args().collect::<Vec<_>>();
    println!("{:?}", vec);
    let num = FileNum::from(vec);
    println!("{}", num);
}
