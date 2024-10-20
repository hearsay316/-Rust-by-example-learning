fn main() {
    let number = 12;
    println!("tell me about {}", number);
    // 匹配到了 就不走了
    match number {
        1 => println!("one"),
        2 | 3 | 4 | 5 | 12 => println!("this is a prime"),
        12..=19 => println!("an't special"),
        _ => {
            println!("这个是一个最后的分支");
        }
    }

    let boolean = true;
    let binary = match boolean {
        false => 1,
        true => 0,
    };
    println!("{}->{}", boolean, binary);
}
/*

解构
match 代码块能以多种方式解构物件。

解构元组
解构枚举
解构指针
解构结构体 match 代码块能以多种方式解构物件

 解构元组 解构 指针 解构 结构体
*/
