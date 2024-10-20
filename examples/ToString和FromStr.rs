use std::fmt;

// ToString 类型和 FromStr
// 要把任何类型转成 string, 只需要实现那个类型的 Tostring  trait  ,然而不要直接那么做, 您应该
// 实现 fmt::Display trait ,他会自动 提供 ToString, 并且还可以用来打印类型, 就像 print! 一节中讨论的那样
struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}
fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle);
    println!("{}", circle.to_string());
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum : {:?}", sum);

    // 变量绑定
    let x = 5;
    // 表达式
    x;
    x + 1;
    15;
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        x_cube + x_squared + x
    };
    let z = {
        2 * x;
    };
    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {:?}", z);
}

// FromStr
