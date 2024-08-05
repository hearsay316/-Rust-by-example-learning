use std::mem;

fn main(){
    let color = String::from("green");
    let print = ||println!("`color` :{}",color);
    print();

    let _reborrow = &color;
    // 当前有两个 借用不可变引用color
    print();
    //  在最后使用 `print`之后, 移动或者重新借用都是允许的
    let _color_moved = color;
    let  mut count = 0;
    let mut inc = ||{
        count+=1;
        println!("`count` :{}", count);
    };
    inc();
    // let mut _reborrow = &mut count;
    inc();
    // println!("__reborrow : {_reborrow}");
    let movavle = Box::new(3);
    let consume = ||{
        println!("movable :{:?}",movavle);
        mem::drop(movavle);
    };
    consume();
    consume();
}