use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

fn build(options: Option<Options>) -> (HashMap<String, Vec<String>>, Vec<String>, String) {
    // let options = options
    let alias = options
        .as_ref()
        .and_then(|opts| opts.alias.as_ref())
        .cloned()
        .unwrap_or_default();
    let modules = options
        .as_ref()
        .and_then(|opts| opts.modules.as_ref())
        .cloned()
        .unwrap_or_else(|| vec!["node_modules".into(), "web_modules".into()]);
    let cwd = options
        .as_ref()
        .and_then(|opts| opts.cwd.clone())
        .map(|cwd| PathBuf::from(cwd).join("").to_string_lossy().into_owned())
        .unwrap_or_default();
    (alias, modules, cwd)
}
fn build4(options: Option<Options>) -> (HashMap<String, Vec<String>>, Vec<String>, String) {
    options
        .and_then(|opts| {
            (
                opts.alias.unwrap_or_default(),
                opts.modules
                    .unwrap_or_else(|| vec!["node_modules".into(), "web_modules".into()]),
                opts.cwd
                    .map(|cwd| PathBuf::from(cwd).join("").to_string_lossy().into_owned())
                    .unwrap_or_default(),
            )
                .into()
        })
        .unwrap_or_else(|| {
            (
                HashMap::default(),
                vec!["node_modules".into(), "web_modules".into()],
                String::default(),
            )
        })
}
fn build5(options: Option<Options>) -> (HashMap<String, Vec<String>>, Vec<String>, String) {
    Some(options.unwrap_or_else(|| Options {
        alias: Some(HashMap::default()),
        modules: Some(Vec::default()),
        cwd: Some(String::default()),
    }))
    .and_then(|opts| {
        Some((
            opts.alias.clone().unwrap_or_default(),
            opts.modules
                .clone()
                .unwrap_or_else(|| vec!["node_modules".into(), "web_modules".into()]),
            opts.cwd
                .clone()
                .map(|cwd| PathBuf::from(cwd).join("").to_string_lossy().into_owned())
                .unwrap_or_default(),
        ))
    })
    .unwrap()
}
fn build3(options: Option<Options>) -> (HashMap<String, Vec<String>>, Vec<String>, String) {
    options
        .and_then(|opts| {
            Some((
                opts.alias.unwrap_or_default(),
                opts.modules
                    .unwrap_or_else(|| vec!["node_modules".into(), "web_modules".into()]),
                opts.cwd
                    .map(|cwd| PathBuf::from(cwd).join("").to_string_lossy().into_owned())
                    .unwrap_or_default(),
            ))
        })
        .unwrap()
}

fn build2(options: Option<Options>) -> (HashMap<String, Vec<String>>, Vec<String>, String) {
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
            cwd: Some(ref cwd), ..
        }) => {
            // 假设 path 是一个已经定义的字符串变量
            let path = "";
            let path_buf = PathBuf::from(cwd).join(path);
            path_buf.to_string_lossy().into_owned()
        }
        _ => String::new(),
    };
    (alias, modules, cwd)
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
/// build  Function executed in: 972.4µs
/// build2 Function executed in: 957.9µs
/// build3 Function executed in: 1.0364ms
/// build4 Function executed in: 952.4µs
fn main() {
    let options = Some(Options {
        alias: None,
        modules: None,
        cwd: None,
    });
    let start1 = Instant::now();
    for _ in 0..10000 {
        build(options.clone());
    }

    let duration1 = start1.elapsed();
    println!("Function executed in: {:?}", duration1);
    // let start2 = Instant::now();
    // build2(options.clone());
    // let duration2 = start2.elapsed();
    // println!("Function executed in: {:?}", duration2);
    // let start3 = Instant::now();
    // build4(options.clone());
    // let duration3 = start3.elapsed();
    // println!("Function executed in: {:?}", duration3);
    // let start4 = Instant::now();
    // build3(options.clone());
    // let duration4 = start4.elapsed();
    // println!("Function executed in: {:?}", duration4);
    // println!("{:?}",options);
}
