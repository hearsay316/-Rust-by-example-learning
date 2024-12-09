use std::process::Command;

fn main() {
    let output = Command::new("./untitled.exe")
        .arg("./src")
        .arg("mp3")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        print!("返回数据:{}", s);
        let arr = s.split_terminator("\n").collect::<Vec<&str>>();
        println!("{:?}", arr.len());
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("rustc failed and stderr was:\n{}", s);
    }
}
