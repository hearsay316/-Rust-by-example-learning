use std::fmt;
use std::fmt::{Display, Result};

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (a_int, b_bool) = pair;
    (b_bool, a_int)
}
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);
fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64,
                      'a', true);
    // 通过元组的下标来访问具体的值
    println!("long tuple first value{}", long_tuple.0);
    println!("long tuple second value:{}", long_tuple.1);
    // 元组也可以充当元组
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);
    // 元组也能打印
    println!("tuple of tuples {:#?}", tuple_of_tuples);

    // 但是很长的元组不能打印(超过12个就不行了)
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple{:?}", too_long_tuple);
    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reverse pair is {:?}", reverse(pair));
    println!("one element tuple :{:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?} ,{:?},{:?},{:?}", a, b, c, d);
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("matrix:{:?}", matrix);
    println!("{}",matrix);
    println!("{}",transpose(matrix));
}
//复习：在上面的例子中给 Matrix 结构体 加上 fmt::Display trait，这样当你从 Debug 格式化 {:?} 切换到 Display 格式化 {} 时，会得到如下的输出：
//
//
// ( 1.1 1.2 )
// ( 2.1 2.2 )

impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result {
        write!(f, "({} {}) \n({} {})", self.0, self.1, self.2, self.3)
    }
}

//以 reverse 函数作为样板，写一个 transpose 函数，
// 它可以接受一个 Matrix 作为参数，
// 并返回一个右上 - 左下对角线上的两元素交换后的 Matrix。
// 举个例子：
// println!("Matrix:\n{}", matrix);
// println!("Transpose:\n{}", transpose(matrix));
// 输出结果：
// Matrix:
// ( 1.1 1.2 )
// ( 2.1 2.2 )
// Transpose:
// ( 1.1 2.1 )
// ( 1.2 2.2 )
fn transpose(tr:Matrix)->Matrix{
    let Matrix(a,b,c,d) = tr;
    Matrix(a,c,b,d)
}
// 这个打字真的好无聊  还是喜欢二指禅 ,那种方式比较好,你们觉得呢?已经可以这样说了吧use  display fmt  cargo run