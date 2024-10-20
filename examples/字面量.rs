fn main() {
    //  带后缀的字面量 ,其类型 在初始化时已经知道了
    // let x = 1i128;
    let x = 1u16;
    let y = 2u64;
    let z = 3f32;
    let a = 2.01f64;
    println!("a -> x -> y-> z-> : {} {} {} {}", a, z, y, z);
    let i = 1;
    let f = 1.0;
    // size_of_val 返回一个变量所占用的字节数
    println!("size of `x` in bytes:{}", std::mem::size_of_val(&x));
    println!("size of `x` in bytes:{}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes:{}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes:{}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes:{}", std::mem::size_of_val(&f));
    println!("size of `a` in bytes:{}", std::mem::size_of_val(&a));
}
