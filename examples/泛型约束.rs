use std::fmt::{Debug, Display, Formatter, Result};
// 定于 一个函数 `printer` ,接受一个类型为泛型`T` 的参数
// trait MyDisplay: Display {}
//
// impl<T: Display> MyDisplay for T {}
//
//
// struct S<T: MyDisplay>(Vec<T>);
// impl<T: MyDisplay> Display for S<T> {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "这个是一个vec[")?;
//
//         for (index, item) in self.0.iter().enumerate() {
//             if index != self.0.len() - 1 {
//                 write!(f, "{},", item)?;
//             } else {
//                 write!(f, "{}", item)?;
//             }
//         }
//         write!(f, "]")
//     }
// }
fn printer<T: Debug>(t: T) {
    println!("{:?}", t);
}
#[derive(Debug)]
struct S<T: Debug>(Vec<T>);

fn main() {
    // 报错！`Vec<T>` 未实现 `Display`。此次泛型具体化失败。
    let s = S(vec![11, 2, 3]);
    printer(s);
}
