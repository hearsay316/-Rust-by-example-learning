// 可以先声明 变量绑定,后面才将他们初始化, 但是这样的方式比较少用,因为这样可能导致使用未初始化的变量.\
fn main() {
    let a_binding ;
    {
        let x = 2;
        a_binding = x*x;
        println!("a binding: {}",a_binding);
    }
    println!("a binding: {}",a_binding);
    let another_binding;
    // println!("another binding: {}",another_binding);
    another_binding = 1;
    println!("another binding : {}",another_binding);
}