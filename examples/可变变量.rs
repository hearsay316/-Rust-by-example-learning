// 变量绑定默认是不可变的 ,但是加上mut 修饰语句后变量就可以改变
fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    println!("Before mutation:{}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation :{}", mutable_binding);
    // _immutable_binding+=1;
    println!("{}", _immutable_binding);
}
