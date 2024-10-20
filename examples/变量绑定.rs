fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = (); //单元类型
    let copied_integer = an_integer;
    println!("An integer :{}", copied_integer);
    println!("A boolean:{}", a_boolean);
    println!("Meet the unit value :{:?}", unit);
    //编译器会对未使用的变量产生警告;可以给变量名前加上下划线前缀来消除警告
    let _unused_variable = 3u32;
    // 改正 在变量名前加上下划线以消除警告
    let noisy_unused_variable = 2u32;
}
