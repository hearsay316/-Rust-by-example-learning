/*use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Error;
use std::io::prelude::*;
#[cfg(target_family = "unix")]
use std::os::unix;
#[cfg(target_family = "windows")]
use std::os::windows;
use std::path::Path;

// %cat path 的简单实现
fn cat (path:&Path)->io::Result<String>{
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s){
        Ok(_)=>Ok(s),
        Err(e)=>Err(e)
    }
}
fn echo(s:&str,path:&Path)->io::Result<()>{
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())
}

fn touch(path:&Path)->io::Result<()>{
    match OpenOptions::new().create(true).write(true).open(path){
        Ok(_)=>Ok(()),
        Err(e)=>Err(e)
    }
}
fn main(){
    let why_fn = move |why:Error|{
        println!("{:?}",why.kind());
    };
    println!("`mkdir a`");
    // Create a directory, returns `io::Result<()>`
    // 创建一个目录，返回`io:：Result<（）>`
    match fs::create_dir("a"){
        Err(why)=>why_fn(why),
        Ok(_)=>{}
    }
    println!("`echo hello > a/b.txt`");
    // The previous match can be simplified using the `unwrap_or_else` method
    // 可以使用“unwrap_or_else”方法简化之前的匹配
    echo("hello",&Path::new("a/b.txt")).unwrap_or_else(why_fn);

    println!("`mkdir -p a/c/d`");
    // Recursively create a directory, returns `io::Result<()>`
    // 递归创建一个目录，返回`io:：Result<（）>`
    fs::create_dir_all("a/c/d").unwrap_or_else(why_fn);

    println!("touch `a/c/e.txt`");
    touch(&Path::new("a/c/e.txt")).unwrap_or_else(why_fn);

    println!("`ln -s ../b.txt a/c/b.txt`");
    #[cfg(target_family = "unix")]{
        unix::fs::symlink("../b.txt","a/c/b.txt").unwrap_or_else(|why|{
            println!("!{:?}",why.kind());
        });
    }
    // #[cfg(target_family = "unix")] {
    //     unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
    //         println!("! {:?}", why.kind());
    //     });
    // }
    // #[cfg(target_family = "windows")]{
    //     windows::fs::symlink_file("../b.txt","a/c/b.txt").unwrap_or_else(|why|{
    //         println!("!{:?}",why.to_string());
    //     });
    // }
    #[cfg(target_family = "windows")] {
        windows::fs::symlink_file("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
            println!("! {:?}", why.to_string());
        });
    }

    println!("`cat a/c/b.txt`");
    match cat(&Path::new("a/c/b.txt")){
        Err(why)=>why_fn(why),
        Ok(s)=>println!(">{}",s),
    }

    println!("`ls s`");
    // Read the contents of a directory, returns `io::Result<Vec<Path>>`
    // 读取目录的内容，返回`io:：Result<Vec<Path>>`
    match fs::read_dir("a"){
        Err(why)=>why_fn(why),
        Ok(paths)=>for path in paths{
            println!("> {:?}",path.unwrap().path());
        }
    }
    println!("`rm a/c/e.txt");
    // Remove a file, returns `io::Result<()>`
    // 删除文件，返回`io:：Result<（）>`
    fs::remove_file("a/c/e.txt").unwrap_or_else(why_fn);


    println!("`rmdir a/c/d`");
    // Remove an empty directory, returns `io::Result<()>`
    // 删除一个空目录，返回`io:：Result<（）>`
    fs::remove_dir("a/c/d").unwrap_or_else(why_fn);
}*/

use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
#[cfg(target_family = "unix")]
use std::os::unix;
#[cfg(target_family = "windows")]
use std::os::windows;
use std::path::Path;

// A simple implementation of `% cat path`
fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// A simple implementation of `% echo s > path`
fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;

    f.write_all(s.as_bytes())
}

// A simple implementation of `% touch path` (ignores existing files)
fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn main() {
    println!("`mkdir a`");
    // Create a directory, returns `io::Result<()>`
    match fs::create_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }

    println!("`echo hello > a/b.txt`");
    // The previous match can be simplified using the `unwrap_or_else` method
    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`mkdir -p a/c/d`");
    // Recursively create a directory, returns `io::Result<()>`
    fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`touch a/c/e.txt`");
    touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`ln -s a/b.txt a/c/b.txt`");
    // Create a symbolic link, returns `io::Result<()>`
    #[cfg(target_family = "unix")]
    {
        unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }
    fs::symlink_metadata("a/b.txt").unwrap_or_else(|why| {
        println!("!qqq {:?}", why);
        panic!("!{:?}", why)
    });
    #[cfg(target_family = "windows")]
    {
        windows::fs::symlink_file("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
            println!("!ssss {:?}", why);
        });
    }

    println!("`cat a/c/b.txt`");
    match cat(&Path::new("a/c/b.txt")) {
        Err(why) => println!("!err {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }

    println!("`ls a`");
    // Read the contents of a directory, returns `io::Result<Vec<Path>>`
    match fs::read_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                println!("> {:?}", path.unwrap().path());
            }
        }
    }

    // println!("`rm a/c/e.txt`");
    // // Remove a file, returns `io::Result<()>`
    // fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
    //     println!("! {:?}", why.kind());
    // });

    // println!("`rmdir a/c/d`");
    // // Remove an empty directory, returns `io::Result<()>`
    // fs::remove_dir("a/c/d").unwrap_or_else(|why| {
    //     println!("! {:?}", why.kind());
    // });
}
