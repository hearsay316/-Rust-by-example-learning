use std::time::Instant;

fn main() {
    let start = Instant::now(); // 记录开始时间
    let ass = foo(10);
    println!("{}", ass);
    let duration = start.elapsed(); // 计算经过的时间
    println!("Time elapsed in foo() is: {:?}", duration);
}

fn foo(i: i32) -> i32 {
    if i == 0 {
        return 0;
    }
    if i == 1 {
        return 1;
    }
    foo(i - 1) + foo(i - 2)
}
/*

thread 'main' has overflowed its stack
error: process didn't exit successfully: `target\debug\examples\异步递归.exe` (exit code: 0xc00000fd, STATUS_STACK_OVERFLOW)


*/
