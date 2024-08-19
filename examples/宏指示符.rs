macro_rules! create_function{
    ($func_name:ident)=>(
        fn $func_name(){
            //`stringify!`  宏把`ident` 转成字符串
            println!("You called {:?}()",stringify!($func_name));
        }
    )
}
create_function!(foo);

create_function!(bar);

macro_rules! print_result{
    ($expression:expr)=>(
        println!("{:?}={:?}",stringify!($expression),$expression);
    )
}
fn main(){
    foo();
    bar();
    print_result!(1u32+1);
    print_result!({
        let x = 1u32;
        x*x+2*x-1
    });

}
// 全部指示符号
// block 块
// expr  用于表达式
// ident 用于变量名或者函数名
// item 语法项
// literal 常用于 字面常量
// pat 模式
// path 路径
// stmt 表达式声明 不包括任何尾随分号 (比较难用)
// tt  标记树
// ty .类型
// vis  可见性描述符