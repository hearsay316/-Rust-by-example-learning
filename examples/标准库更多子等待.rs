use std::process::Command;

fn main(){
    let mut child  = Command::new("rustc").arg("--version").spawn().unwrap();
    let _result  = child.wait().unwrap();
    println!("reached end of main")
}