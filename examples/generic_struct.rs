// 定义一个包含泛型的结构体
struct Point<T> {
    x: T,
    y: T,
}

// 定义一个包含多个泛型参数的结构体
struct Pair<T, U> {
    first: T,
    second: U,
}

// 为泛型结构体实现方法
impl<T> Point<T> {
    // 构造函数
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

// 为特定类型实现特定的方法
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // 使用整数类型的 Point
    let integer_point = Point { x: 5, y: 10 };
    println!("Integer point: x = {}, y = {}", integer_point.x, integer_point.y);

    // 使用浮点数类型的 Point
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("Float point: x = {}, y = {}", float_point.x, float_point.y);
    println!("Distance from origin: {}", float_point.distance_from_origin());

    // 使用构造函数创建 Point
    let point = Point::new(3, 7);
    println!("Created point: x = {}, y = {}", point.x, point.y);

    // 使用不同类型的 Pair
    let pair = Pair {
        first: 1,
        second: "hello",
    };
    println!("Pair: first = {}, second = {}", pair.first, pair.second);
}
