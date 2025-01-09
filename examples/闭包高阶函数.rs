fn is_odd(n: u32) -> bool {
    n % 2 == 1
}
fn main() {
    println!("求1000以下所有平方奇数的和");
    let upper = 1000;
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        println!("{}", n_squared);
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style :{}", acc);
    // Rust 的基本类型包括:
    // 整数类型: i8, i16, i32, i64, i128, isize
    //          u8, u16, u32, u64, u128, usize 
    // 浮点类型: f32, f64
    // 布尔类型: bool
    // 字符类型: char
    // 单元类型: () (unit type)
    let sum_of_squared_odd_unmbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < upper)
        .filter(|&n| is_odd(n))
        .fold(0, |sum, i| sum + i);
    println!("functional style : {}", sum_of_squared_odd_unmbers);
}
