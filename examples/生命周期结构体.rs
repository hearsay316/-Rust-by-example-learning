#![allow(dead_code)]

#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// 和前面的类似,这里引用都需要比这个结构体长寿
#[derive(Debug)]
struct NameBorrowed<'a>{
    x:&'a i32,
    y:&'a i32
}

#[derive(Debug)]
enum Either<'a>{
    Num(i32),
    Ref(&'a i32)
}
fn main (){
    let x = 18;
    let y = 15;
    let single = Borrowed(&x);
    let double = NameBorrowed{
        x:&x,
        y:&y
    };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);
    println!("x is borrowed in {:?}",single.0);
    println!("x and y are borrowed in {:?}",double);
    println!("x is borrowed in {:?}",reference);
    println!("y is *not* borrowed in {:?}",number);

}
/*

warning: fields `x` and `y` are never read
field `0` is never readn by default

   |     field in this variant
   |
help: consider changing the field to be of unit type to suppress this warning while preserving the field numbering, or remove the field


*/