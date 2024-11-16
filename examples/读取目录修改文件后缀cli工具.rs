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
            update_file_name_suffix(PathBuf::from(&self.path), &self.suffix);
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
                update_file_name_suffix(file_path, suffix);
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
fn update_file_name_suffix(entry: PathBuf, suffix: &str) {
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
#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn test_get_path_file() {
        let path_dir = "./test1";
        let path_file = "1.mp4";
        let path = format!("{path_dir}/{path_file}");
        create_dir_files_v2(vec![&path]);
        assert_eq!(get_path_file(&path), true);
        delete_dir(path_dir);
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
        update_file_name_suffix(PathBuf::from(path), new_suffix);
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
        assert_eq!(options.path, "D:\\video\\2023-05-01\\1.mp4".to_string());
        assert_eq!(options.suffix, "mp4".to_string());
    }

    #[test]
    fn test_update_path_name_suffix() {
        let path_dir = "./test3";
        let path_file = "1";
        let old_suffix = "mp3";
        let new_suffix = "mp4";
        let new_path = format!("{path_dir}/{path_file}.{new_suffix}");
        create_dir_files(path_dir, vec![&format!("{path_file}.{old_suffix}")]);
        update_path_name_suffix(&path_dir, new_suffix);
        assert_eq!(get_path_file(&new_path), true);
        delete_dir(path_dir);
    }

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
        let dir_data = DirData::new(&vec!["./test/1.txt", "/test/2.txt", "./sss.ts"]);
        dir_data.create_dir_data();
        dir_data.delete_dir_data();
        println!("{:?}", dir_data);
    }
    #[derive(Debug)]
    struct DirData {
        dir_path: HashSet<String>,
        file_vec: HashSet<String>,
    }
    impl DirData {
        fn new(vec: &Vec<&str>) -> Self {
            let mut dir_path = HashSet::new();
            let mut file_vec = HashSet::new();
            for path in vec {
                if !path.contains("/") {
                    continue;
                }
                let dir_path_arr = path.split("/").collect::<Vec<_>>();
                if dir_path_arr[0] == "." && dir_path_arr.len() == 2 {
                    file_vec.insert(path.to_string());
                    continue;
                }
                if dir_path_arr[0] == "." && dir_path_arr.len() > 2 {
                    dir_path.insert(dir_path_arr[0..2].join("/"));
                    file_vec.insert(path.to_string());
                    continue;
                };
                dir_path.insert(format!(".{}", dir_path_arr[0..2].join("/")));
                file_vec.insert(format!(".{}", path));
            }
            Self { dir_path, file_vec }
        }
        fn create_dir_data(&self) {
            self.dir_path
                .iter()
                .for_each(|dir| match fs::create_dir_all(dir) {
                    Ok(_) => println!("目录创建成功或已存在"),
                    Err(e) => println!("创建目录时发生错误: {:?}", e),
                });
            self.file_vec
                .iter()
                .for_each(|file| match fs::File::create(file) {
                    Ok(_) => println!("文件创建成功"),
                    Err(e) => println!("创建文件时发生错误: {:?}", e),
                });
        }
        fn delete_dir_data(&self) {
            self.file_vec
                .iter()
                .for_each(|file_path| match fs::remove_file(file_path) {
                    Ok(_) => println!("删除文件成功"),
                    Err(e) => println!("删除文件时发生错误: {:?}", e),
                });
            self.dir_path
                .iter()
                .for_each(|path| match fs::remove_dir_all(path) {
                    Ok(_) => println!("删除目录成功"),
                    Err(e) => println!("删除目录时发生错误: {:?}", e),
                });
        }
    }
}
