struct Point {
    x: f64,
    y: f64
}
impl Point{
    // 这个是一个静态方法 (static method)
    // 静态方法不需要被实例调用
    // 这类方法一般作构造器
    fn origin() ->Point {
        Point{
            x:0.0,
            y:0.0
        }
    }
    fn new(x:f64,y:f64)->Point{
        Point{
            x,y
        }
    }
}
struct Rectangle {
    p1:Point,
    p2:Point
}
impl Rectangle {

}