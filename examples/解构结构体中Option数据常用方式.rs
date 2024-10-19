use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

fn build(options: Option<Options>) {
    // let options = options
    let alias = options.as_ref().and_then(|opts| opts.alias.as_ref()).cloned().unwrap_or_default();
    let modules = options.as_ref().and_then(|opts| opts.modules.as_ref()).cloned().unwrap_or_else(|| vec!["node_modules".into(), "web_modules".into()]);
    let cwd = options.as_ref().and_then(|opts| opts.cwd.clone()).map(|cwd| PathBuf::from(cwd).join("").to_string_lossy().into_owned()).unwrap_or_default();
    println!("{:?}", alias);
    println!("{:?}", modules);
    println!("{:?}", cwd);
    println!("{:?}", options);
}


fn build3(options: Option<Options>) {
    let (alias, modules, cwd) = options.as_ref()
        .and_then(|opts| Some((
            opts.alias.clone().unwrap_or_default(),
            opts.modules.clone().unwrap_or_else(|| vec!["node_modules".into(), "web_modules".into()]),
            opts.cwd.clone().map(|cwd| PathBuf::from(cwd).join("").to_string_lossy().into_owned()).unwrap_or_default()
        )))
        .unwrap();
    println!("{:?}", alias);
    println!("{:?}", modules);
    println!("{:?}", cwd);
    println!("{:?}", options);
}

fn build2(options: Option<Options>) {
    let alias: HashMap<String, Vec<String>> = match options {
        Some(Options {
                 alias: Some(ref alias),
                 ..
             }) => alias.clone(),
        _ => HashMap::new(),
    };

    let modules: Vec<String> = match options {
        Some(Options {
                 modules: Some(ref modules),
                 ..
             }) => modules.clone(),
        _ => vec!["node_modules".into(), "web_modules".into()],
    };

    let cwd: String = match options {
        Some(Options {
                 cwd: Some(ref cwd),
                 ..
             }) => {
            // 假设 path 是一个已经定义的字符串变量
            let path = "";
            let path_buf = PathBuf::from(cwd).join(path);
            path_buf.to_string_lossy().into_owned()
        }
        _ => String::new(),
    };
    println!("{:?}", alias);
    println!("{:?}", modules);
    println!("{:?}", cwd);
    println!("{:?}", options);
}
// fn  path_json(s:String)-> PathBuf{
//     let mut path = PathBuf::from(r"C:\");
//     path.push(s);
//     path
// }
// 假设的 Options 结构体
#[derive(Debug, Clone)]
struct Options {
    alias: Option<HashMap<String, Vec<String>>>,
    modules: Option<Vec<String>>,
    cwd: Option<String>,
}

fn main() {
    let options = Some(Options {
        alias: None,
        modules: None,
        cwd: None,
    });
    let start = Instant::now();
    build3(options.clone());
    let duration = start.elapsed();
    println!("Function executed in: {:?}", duration);

    let start = Instant::now();
    build3(options.clone());
    let duration = start.elapsed();
    println!("Function executed in: {:?}", duration);
    let start = Instant::now();
    build3(options.clone());
    let duration = start.elapsed();
    println!("Function executed in: {:?}", duration);
    // println!("{:?}",options);
}