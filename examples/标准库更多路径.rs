use std::path::Path;

fn main(){
    let path  = Path::new(".");
    let display = path.display();
    println!("{:?}",display);
    let new_path = path.join("a").join("b");
    match new_path.to_str(){
        None=>println!("new path os not a valid UTF-8 sequence"),
        Some(s)=>println!("new path is {}",s==new_path.display().to_string()),
    }
    println!("{}",new_path.display());
}