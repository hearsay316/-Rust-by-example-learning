use std::num::ParseIntError;

fn double_first(vec:Vec<&str>) ->Option<Result<i32,ParseIntError>>{
    vec.first().map(|first|{
        first.parse::<i32>().map(|n| 2 * n)
    })
}
fn main(){
    let numbers = vec!["42","92","18"];
    let empty = vec![];
    let strings = vec!["tofu","93","18"];

    println!("The first doubled is {:?}",double_first(numbers).unwrap().unwrap());
    println!("The first doubled is {:?}",double_first(empty));
    println!("The first doubled is {:?}",double_first(strings));
}

fn double_first_1(vec:Vec<&str>)->Result<Option<i32>,ParseIntError>{

    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n|n*2)
    });
    // Option<Result<i32, ParseIntError>>
    opt.map_or(Ok(None),|r|r.map(Some))
}