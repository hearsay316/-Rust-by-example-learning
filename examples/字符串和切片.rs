// 定义主函数
fn main() {
    // 创建一个字符串 s，值为 "hhh"
    let s = "hhh";

    // 克隆字符串 s 到 s1，s1 和 s 是完全独立的数据
    let s1 = s.clone();

    // 将字符串 s 的引用赋值给 s2，s2 和 s 共享数据
    let s2 = s;

    // 创建一个字符串切片 s3，包含 s 的前两个字符
    let s3 = &s[0..2];

    // 将字符串 s 转换为 String 类型的 s4
    let s4 = s.to_string();

    // 打印 s1 的地址和值
    // 注意 s1 和 s 是完全独立的数据，它们的地址不同
    println!("{:p}--{}", s1, s1);

    // 打印 s2 的地址和值，注意 s2 和 s 共享相同的内存地址
    println!("{:p}--{}", s2, s2);

    // 打印 s3 的地址和值，s3 是一个切片，它的地址与 s 的地址不同
    println!("{:p}--{}", s3, s3);

    // 打印 s 的地址和值
    println!("{:p}--{}", s, s);

    // 打印 s4 的地址（通过转换为 *const _）和值，s4 是 String 类型，其内存地址与 s 不同
    println!("{:p}--{}", &s4 as *const _, s.to_string());
}
