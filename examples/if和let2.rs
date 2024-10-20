// 该枚举故意未注明 `#[derive(PartialEq)]`，
// 并且也没为其实现 `PartialEq`。这就是为什么下面比较 `Foo::Bar==a` 会失败的原因。

#![allow(dead_code)]

fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }
    //  如果指向失败 , 可以用 else
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }
    // 提供另外一种失败情况下的条件
    let l_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched :{:?}", i);
    } else if l_like_letters {
        println!("Didn't match  a number . Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emotion :)!");
    }
}
