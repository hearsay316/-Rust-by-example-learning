fn apply<F>(f: F)
where
    // 闭包没有输入值和返回值。
    F: FnOnce(),
{
    f();
}
//   输入闭包,返回一个i\32的整数型的函数
fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}
fn main() {
    let greeting = "hello";
    let mut farwell = "goodbye".to_owned();
    let diary = || {
        println!("I said {}.", greeting);
        farwell.push_str("!!!");
        println!("Then I screamed {}", farwell);
        println!("Now I can sleep. zzzzz");
        std::mem::drop(farwell);
    };
    apply(diary);
    // println!("{farwell:?}");
    let double = |x| 2 * x;
    println!("3 doubled : {}", apply_to_3(double));
}
