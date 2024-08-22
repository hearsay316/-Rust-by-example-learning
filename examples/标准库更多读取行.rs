use std::fs::File;
use std::io;
use std::io::{BufRead, Read};
use std::path::Path;

fn main(){
    if let Ok(lines) = read_lines("./hosts"){
        for line in lines {
            if let Ok(ip) = line{
                println!("{}",ip);
            }
        }
    }
}
fn read_lines<P>(filename:P)->io::Result<io::Lines<io::BufReader<File>>>
where P:AsRef<Path>,{
    let mut file = File::open(filename)?;
    // let mut s = String::new();
    // file.read_to_string(&mut s).unwrap();
    // println!("555{:?}",s);
    Ok(io::BufReader::new(file).lines())
}