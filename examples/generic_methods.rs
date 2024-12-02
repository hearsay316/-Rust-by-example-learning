// 一个普通的结构体，没有泛型
struct Calculator {
    value: i32,
}

impl Calculator {
    fn new(value: i32) -> Calculator {
        Calculator { value }
    }

    // 泛型方法，可以接受任何实现了 Display 特征的类型
    fn display<T: std::fmt::Display>(&self, message: T) {
        println!("Value is: {}, Message is: {}", self.value, message);
    }

    // 泛型方法，可以处理任何数字类型
    fn add<T: std::ops::Add<Output = T> + Copy>(&self, x: T, y: T) -> T {
        x + y
    }

    // 泛型方法，带有多个类型参数
    fn convert<T, U>(&self, value: T) -> U 
    where 
        T: std::fmt::Display,
        U: std::fmt::Display + From<T>,
    {
        U::from(value)
    }

    // 泛型方法，返回一个 Option
    fn find_in_slice<T: PartialEq>(&self, slice: &[T], item: T) -> Option<usize> {
        slice.iter().position(|x| *x == item)
    }
}

fn main() {
    let calc = Calculator::new(42);

    // 使用display方法，可以传入不同类型
    calc.display("Hello");  // 字符串
    calc.display(123);      // 数字
    calc.display(true);     // 布尔值

    // 使用add方法
    let sum_i32 = calc.add(10, 20);
    let sum_f64 = calc.add(10.5, 20.5);
    println!("Sum i32: {}", sum_i32);
    println!("Sum f64: {}", sum_f64);

    // 使用convert方法
    let int_val: i32 = 42;
    let float_val: f64 = calc.convert(int_val);
    println!("Converted value: {}", float_val);

    // 使用find_in_slice方法
    let numbers = vec![1, 2, 3, 4, 5];
    if let Some(pos) = calc.find_in_slice(&numbers, 3) {
        println!("Found 3 at position: {}", pos);
    }

    let words = vec!["apple", "banana", "cherry"];
    if let Some(pos) = calc.find_in_slice(&words, "banana") {
        println!("Found 'banana' at position: {}", pos);
    }
}
