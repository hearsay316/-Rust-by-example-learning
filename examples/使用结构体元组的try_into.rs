use std::fs::{read_dir, DirEntry};
use std::io;

struct W<T>(T);
impl TryFrom<W<&DirEntry>> for String {
    type Error = io::Error;
    fn try_from(value: W<&DirEntry>) -> Result<Self, Self::Error> {
        value
            .0
            .path()
            .to_str()
            .map(String::from)
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "路径无法转换为字符串"))
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("hello world");
    for entry in read_dir("../")?.filter_map(|e| e.ok()) {
        let entry: String = W(&entry).try_into()?;
        println!("{entry}");
    }
    Ok(())
}
