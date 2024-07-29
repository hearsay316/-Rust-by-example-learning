type NanSecond = u64;
type Inch = u64;
#[allow(non_camel_case_types)]
type u64_t = u64;
fn main(){
    let nanosecond:NanSecond = 5 as u64_t;
    let inches:Inch = 2 as u64_t;
    println!("{} nanosecond + {} inches = {} unit?",nanosecond,inches, nanosecond+inches);
}


/*
类型转换
Rust 使用 trait 解决类型之间的转换问题。
最一般的转换会用到 From 和 Into 两个 trait。
不过，即便常见的情况也可能会用到特别的 trait，
尤其是从 String 转换到别的类型，以及把别的类型转换到 String 时。
*/