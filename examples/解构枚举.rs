// 和前面一样 解构enum enum  的方式如下

struct Pen {
    name: String,
    age: i32,
}
#[allow(dead_code)]
enum Color {
    // 这三个取值仅由他们自己的名字来来指定
    Red,
    Blue,
    Green,
    //  这些则是把u32 元组 赋予不同的名字,以彩色模型命名.
    RBG(u32, u32, u32),
    HSV(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::RBG(122, 17, 40);
    println!("What color is it ");
    match color {
        Color::Red => println!("The color is Red "),
        Color::Blue => println!("The color is Blue"),
        Color::Green => println!("The color is Green"),
        Color::RBG(r, g, b) => println!("Red:{},green:{},blue:{}", r, g, b),
        Color::HSV(r, g, b) => println!("{}  {} {}", r, g, b),
        Color::CMY(r, g, b) => println!("{}  {} {}", r, g, b),
        Color::CMYK(r, g, b, a) => println!("{}  {} {} {}", r, g, b, a),
        _ => println!("这是错误"),
    }
}
