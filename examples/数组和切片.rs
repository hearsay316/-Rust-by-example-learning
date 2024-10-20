// 数组和切片
/*
数组是一组拥有相同类型T的对象的集合, 在内存中是连续存储.
数组是用中括号[]来创建,且他们的大小在编译时 会被确定. 数组的类型标记为[T;N](
注: T为元素类型,length 表示数组大小
)
切片类型和数组类型相似,但其大小在编译时是不确定的,相反,切片是一个双字对象
(two-word-object),第一个字是指向数据的指针,第二个是切片的长度. 这个"字"的宽度和usize相同,
由处理器架构来决定,比如在x86-64平台上就是64位.slice 可以用来借用数组的一部分. slice的标记
类型为&[T]
*/
use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("array is :{:?}", slice);
    println!("first element of the slice:{}", slice[0]);
    println!("the slice has {} element ", slice.len());
}
fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // 所有元素可以初始化成相同的值
    let ys: [i32; 500] = [0; 500];
    println!("打印数组的全部{:?}", ys);
    println!("first element of the array:{}", xs[0]);

    println!("SECOND element of the array:{}", xs[1]);
    println!("array size :{}", xs.len());
    //   数组在栈中分配
    println!("array  occupise {} bytes", mem::size_of_val(&xs));
    // mem::size_of_val(&xx)
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);
    // slice 可以指向数组中的一部分
    println!("borrow a section of the array as a slice ");
    analyze_slice(&xs[1..=4]);
    // 数组越界
    // println!("{}",xs[5]);
}
