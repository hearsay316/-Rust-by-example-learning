fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;
    println!("a={}, b={}, y={}", a, b, y);
    //  可以解构解构体 并 重命名 变量, 成员顺序并不重要
    let Foo { y: i, x: j } = foo;
    println!("i ={}, j={:?}", i, j);
    // 也可以忽略某些变量
    let Foo { y, .. } = foo;
    println!("y = {}", y);
}
