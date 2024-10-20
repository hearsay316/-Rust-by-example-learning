// 变量绑定 有一个作用域 (scope) ,它被限定只在一个代码块(block)中生存(live);
//代码块是一个被{}包围的语句集合.另外也 允许变量遮蔽(variable shadowing);
fn main() {
    let long_lived_binding = 2;
    // println!("inner short :{} ", )
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
        // println!("")
        // 此绑定遮蔽了外边的绑定
        let long_lived_binding = 5_f32;
        println!("inner long:{}", long_lived_binding);
    }
    // 代码块结束
    // 报错! `short_lived_binding`在此作用域上不存在
    // println!("out short :{}",short_lived_binding);
    println!("out long:{} ", long_lived_binding);
    let long_lived_binding = "a";
    println!("out long: {}", long_lived_binding);
}
