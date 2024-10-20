// from 和Into 两个 trait 是内部关联的, 实际上这是他们实现的一部分.  如果我们能够从类型B 得到类型A , 那么很容易相信
// 我们也能够把类型B 转换成类型A

// fn  main(){
//     let my_str = "hello";
//     let my_string = String::from(my_str);
//     println!("这个是测试 {}",my_string);
// }
#[derive(Debug)]
struct Number {
    value: i32,
}
impl Number {
    fn new(item: i32) -> Self {
        Number { value: item }
    }
}
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number::new(item)
    }
}
fn main() {
    let num = Number::from(30);
    println!("My Number is : {:?}", num);
    let num2 = 5;
    let num_number: Number = num2.into();
    println!("num_number is :{:?} ", num_number);
}
