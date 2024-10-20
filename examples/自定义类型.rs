//自定义类型
// 结构体  struct   struct  struct struct
//枚举 enum enum  enum  enum  enum
// 而常量可以通过const 和static来定义  static static static  const
// struct enum  结构体和枚举
//常量是   const static static

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
//  单元结构体
struct Unit;
// 元组结构体
struct Pair(i32, f32);
//带有两个字段的结构体
#[derive(Debug, Clone)]
struct Point {
    x: f32,
    y: f32,
}

//  结构体可以作为另一个结构体的字段
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
fn main() {
    // 使用简单的写法初始化字段,并创建结构体
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{peter:?}");
    // 初始化结构体 "Point"
    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates:({},{})", point.x, point.y);
    // 使用结构体更新语法创建新出的point
    // 这样可以用到之前的point

    // note: the base struct must always be the last field
    //注意：基本结构必须始终是最后一个字段
    let bottom_right = Point { x: 5.2, ..point };
    println!("{:?} ", bottom_right);

    // `bottom_right.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
    // 解构  改变量名称
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;
    println!("{}___{}", left_edge, top_edge);
    let _rectang = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };
    //  实例话一个单元结束
    let _unit = Unit;
    let pair = Pair(1, 0.1);
    // 访问元组结构体的字段
    println!(" pair contains {:?} and {:?}", pair.0, pair.1);
    let Pair(integer, decimal) = pair;
    println!("{}and{}", integer, decimal);
    println!("{:?}", rect_area(&_rectang));

    // 使用 square 函数创建一个 Rectangle
    let top_left = Point { x: 0.0, y: 5.0 };
    let size = 3.0;
    let square = square(&top_left, size);
    println!("square{:?}", square);
    // 计算并打印这个 Rectangle 的面积
    println!("The area of the square is: {}", rect_area(&square));
    println!("{:?}", top_left);
}

fn rect_area(rectangle: &Rectangle) -> f32 {
    println!("{:?}", rectangle);
    let Rectangle {
        top_left,
        bottom_right,
    } = rectangle;
    let width = bottom_right.x - top_left.x;
    let height = bottom_right.y - top_left.y;
    width * height
}
fn square(top_left: &Point, size: f32) -> Rectangle {
    Rectangle {
        top_left: top_left.clone(),
        bottom_right: Point {
            x: top_left.x + size,
            y: top_left.y + size,
        },
    }
}
