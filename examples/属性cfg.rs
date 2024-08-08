#[cfg(target_os="linux")]
fn are_you_on_linux(){
    println!("当前运行的系统是linux");

}
#[cfg(not(target_os="linux"))]
fn are_you_on_linux(){
    println!("当前运行的系统不是linux");
}

fn main(){
    are_you_on_linux();
    println!("Are you sure?");
    if cfg!(target_os = "windows"){
        println!("是的这个是windwo系统");
    }else {
        println!("这个是liunx系统!");
    }
}

