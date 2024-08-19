
macro_rules! test{
    ($left:expr; and $rigth:expr)=>(
        println!("{:?} and {:?} is {:?}",stringify!($left),stringify!($rigth),$left&&$rigth)
    );
    ($left:expr; or $right:expr)=>(
        println!("{:?} or {:?} is {:?}",
        stringify!($left),
        stringify!($right),
            $left||$right
        )
    )
}
fn main() {
    test!(1i32+1 ==2i32; and 2i32*2==5i32);
    test!(true; or false);
}