use std::marker::PhantomData;

//  生命周期失败
// struct Bounded<'a, 'b: 'a, T: ?Sized>(&'a T, PhantomData<&'b ()>);
//
// fn helper<'a, 'b, T: ?Sized>(
//     input: &'a T,
//     closure: impl FnOnce(&T) -> Bounded<'b, '_, T>,
// ) -> &'b T {
//     closure(input).0
// }
//
// fn extend<'a, 'b, T: ?Sized>(input: &'a T) -> &'b T {
//     helper(input, |x| Bounded(x, PhantomData))
// }

// `Bounded` 结构体用于表示一个受生命周期 `'b` 约束的引用类型
// 其中 `'b: 'a` 表示 `'b` 必须包含 `'a`
// `T: ?Sized` 表示 `T` 可以是未知大小的类型，即可以是 `str` 等
struct Bounded<'a, 'b: 'a, T: ?Sized>(&'a T, PhantomData<&'b ()>);

// `helper` 函数用于执行一个闭包，该闭包接受一个引用并返回一个 `Bounded` 实例
// 该函数的主要作用是通过闭包将输入引用的生命周期转换为指定的生命周期 `'b`
fn helper<'b, T: ?Sized>(
    input: &'b T,                                   // 输入参数，一个引用类型的值
    closure: impl FnOnce(&T) -> Bounded<'b, '_, T>, // 一个闭包，接受一个 `&T` 类型的引用并返回一个 `Bounded` 实例
) -> &'b T {
    closure(input).0 // 调用闭包并返回结果中的引用
}

// `extend` 函数用于延长输入引用的生命周期至 `'b`
// 实际上，该函数通过 `helper` 函数和 `Bounded` 结构体来实现生命周期的转换
fn extend<T: ?Sized>(input: &T) -> &T {
    helper(input, |x| Bounded(x, PhantomData)) // 使用闭包将输入引用包装成 `Bounded` 实例，从而转换生命周期
}

fn main() {
    let s = String::from("aaaa");
    let b = s.clone();
    // 使用 `extend` 函数将 `&str` 类型的引用的生命周期转换为 `'static`
    // 这里利用了 Rust 的生命周期转换规则和 `PhantomData` 来实现
    let a = extend(b.as_str());
    drop(s); // 释放 `s` 的所有权，此时 `s` 不再有效
             // 由于 `a` 的生命周期被标记为 `'static`，理论上它应该在程序的整个生命周期内有效
             // 但实际上，由于 `a` 原本是基于 `s` 的引用，释放 `s` 后使用 `a` 可能导致未定义行为
             // 这里展示了如何通过类型系统操作生命周期，但实际使用时需要谨慎，避免悬挂引用
    println!("{a}"); // <
}
