#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // 这个是一个静态方法 (static method)
    // 静态方法不需要被实例调用
    // 这类方法一般作构造器
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}
impl Rectangle {
    fn area(&self) -> f64 {
        // `self` 通过点运算符来访问结构体字段
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        // `abs` 是一个`f64`类型的方法,返回调用者的绝对值
        ((x1 - x2) * (y1 - y2)).abs()
    }
    // 矩形的周长
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }
    // 这个方法要求调用者是可变的
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p1.y += y;

        self.p2.x += x;
        self.p2.y += y;
    }
    // `pair` 拥有资源:两个堆分配的整型
}
struct Pair(Box<i32>, Box<i32>);
impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying pair ({},{})", first, second);
        // `first` 和 `second` 离开作用域后释放
    }
}
fn main() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    // 实例方法通过点运算符来调用
    // 注意第一个参数 `&self` 是来隐式传递的,亦即:
    println!("Rectangle perimeter:{}", rectangle.perimeter());
    println!("Rectangle area:{}", rectangle.area());
    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // rectangle.translate();

    square.translate(1.0, 1.0);
    println!("square :{:?}", square);
    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}
