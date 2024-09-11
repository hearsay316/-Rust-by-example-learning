use std::{env, fs};
fn help() {
    println!("请输入文件夹和需要修改的后缀,以空格区分");
}
fn  main(){
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3=>{
            let dir = &args[1];
            let suffix = &args[2];
            println!("{:?} --{:?}",dir,suffix);
            dir_file(dir,suffix)
        }
        _=>{
            println!("args is {:?}",args);
            help()
        }
    }
}
//rustc  ./examples/Rust获取目录修改文件名.rs  -C opt-level=3
fn dir_file(path:&str,suffix:&str){
    let entries = fs::read_dir(path).expect("读取文件夹失败");
    for entry in entries {
        if let Ok(entry) = entry{
            let _str = entry.path();
             let mut ren_name =entry.path();
            ren_name.set_extension(suffix);
            println!("{:?}",ren_name);
            fs::rename(_str, ren_name).expect("改名字失败");
        }
    }
    println!("已经修改后缀成功");
}