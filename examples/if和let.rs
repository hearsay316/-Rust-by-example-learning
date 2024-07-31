
// 该枚举故意未注明 `#[derive(PartialEq)]`，
// 并且也没为其实现 `PartialEq`。这就是为什么下面比较 `Foo::Bar==a` 会失败的原因。

#![allow(dead_code)]

#[derive(PartialEq)]
enum Foo {Bar, User }
fn main() {
    let a = Foo::User;

    // 变量匹配 Foo::Bar
    if let Foo::Bar = a {
        // ^-- 这就是编译时发现的错误。使用 `if let` 来替换它。
        println!("a is foobar ");
    }else{
        println!("zzz")
    }
}
/*
C:/Users/hearsay316/.cargo/bin/cargo.exe run --color=always --package Rust-by-example-learning --example if和let
warning: variant `Bar` is never constructed
 --> examples\if和let.rs:5:11
  |
5 | enum Foo {Bar, User }
  |      ---  ^^^
  |      |
  |      variant in this enum
  |
  = note: `Foo` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
warning: `Rust-by-example-learning` (example "if和let") generated 1 warning
the lint level is defined here


*/