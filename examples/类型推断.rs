fn main() {
    /*
    Rust 的类型推断引擎是很聪明的，
    它不只是在初始化时看看右值（r-value）的类型而已，
    它还会考察变量之后会怎样使用，
    借此推断类型。这是一个类型推导的进阶例子：
    */
    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);
}
