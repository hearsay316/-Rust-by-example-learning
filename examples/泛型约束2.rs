use std::fmt::Debug;

trait HasArea{
    fn area(&self)->f64;
}
#[derive(Debug)]
struct Rectangle{
    length:f64,
    height:f64
}
struct Triangle{
    length:f64,
    height:f64
}

impl HasArea for Rectangle{
    fn area(&self)->f64{
        self.length*self.height
    }
}
fn print_debug<T:Debug>(t:T){
    println!("{:?}",t);
}
fn area<T:HasArea>(t:&T)->f64{
    t.area()
}
fn main(){

    let rectangle = Rectangle{
        height:4.0,
        length:3.0
    };
    let _triangle = Triangle{
        height:3.0,
        length:4.0
    };
    print_debug(&rectangle);
    println!("Area: {}",area(&rectangle));
    // print_debug(&_triangle);
    // println!("Area :{} ",area(&_triangle));
}