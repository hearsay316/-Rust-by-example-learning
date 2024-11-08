use std::path::PathBuf;
use std::{env, fs};
#[derive(Debug)]
struct Options {
    path: String,   // 文件路径
    suffix: String, // 后缀格式
}
impl Options {
    fn new(args: Vec<String>) -> Self {
        // 获取当前工作目录
        let current_dir = env::current_dir().expect("获取当前目录失败");
        // 将 PathBuf 转换为字符串
        let current_dir_str = current_dir.to_string_lossy().to_string();
        println!("当前目录 :{:?}", current_dir_str);
        let path = args.get(1).unwrap_or_else(|| &current_dir_str).clone();
        let suffix = args.get(2).unwrap_or(&"mp4".to_string()).clone();
        Options { path, suffix }
    }
    // 修改文件夹后缀
    fn update_name_suffix(&self) {
        if get_path_file(&self.path) {
            update_name_suffix(PathBuf::from(&self.path), &self.suffix);
        } else {
            update_path_name_suffix(&self.path, &self.suffix);
        }
        println!("修改完成");
    }
}
fn update_path_name_suffix(path: &str, suffix: &str) {
    let entries = fs::read_dir(path).expect("读取文件夹失败");
    for entry in entries {
        if let Ok(entry) = entry {
            let file_path = entry.path();
            if file_path.is_file() {
                update_name_suffix(file_path, suffix);
            } else {
                let file_path = file_path.to_string_lossy().to_string();
                update_path_name_suffix(&file_path, suffix);
            }
        }
    }
}
fn get_path_file(path: &str) -> bool {
    let metadata = fs::metadata(path).expect("路径不存在");
    metadata.is_file()
}
fn update_name_suffix(entry: PathBuf, suffix: &str) {
    let mut ren_name = entry.clone();
    let old_name = entry.clone();
    ren_name.set_extension(&suffix);
    println!("{:?}", ren_name);
    println!("正在修改文件名称,{:?},{:?},{}", ren_name, old_name, suffix);
    fs::rename(old_name, ren_name).expect("改名字失败");
    println!("修改 {:?} 成功", entry);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let options = Options::new(args);
    println!("数据是: {:?}", options);
    options.update_name_suffix();
}
