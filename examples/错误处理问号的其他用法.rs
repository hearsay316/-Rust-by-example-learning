use std::{error, fmt};

type Result<T> = std::result::Result<T,Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVec ;

impl fmt::Display for EmptyVec {
    fn fmt(&self,f: fmt::Formatter)->fmt::Result{
        write!(f,"invalid first item do double")
    }

}
impl error::Error for EmptyVec{}

//ok_or  Option 格式 转化成 Result
fn double_first(vec:Vec<&str>)->Result<i32>{
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2*parsed)
}

fn print(result: Result<i32>){
    match result{
        Ok(N)=>println!("The first doubled is {}",n),
        Err(e)=>println!("Error is {}",e)
    }
}

fn main(){

    let numbers = vec!["42","93","18"];
    let empty = vec![];
    let strings = vec!["tofu","98","18"];
    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}