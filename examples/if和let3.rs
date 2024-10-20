// 该枚举故意未注明 `#[derive(PartialEq)]`，
// 并且也没为其实现 `PartialEq`。这就是为什么下面比较 `Foo::Bar==a` 会失败的原因。

#![allow(dead_code)]

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}
fn main() {
    //  创建变量
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar")
    }
    if let Foo::Qux(value) = c {
        println!("c is {} ", value);
    }
}
