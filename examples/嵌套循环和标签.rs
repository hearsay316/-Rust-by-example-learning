fn main() {
    'outer: loop {
        println!("第一层Entered the out loop");
        'inner: loop {
            println!("第二层 Entered the inner loop");
            // break;
            break 'outer;
        }
        println!("这一点永远不会达到 This point will never be reached")
    }
    println!("退出外环 Exited the outer loop")
}
