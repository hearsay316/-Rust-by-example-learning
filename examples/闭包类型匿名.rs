// fn apply (  f:  impl FnOnce())
// {
//     f();
// }

fn apply<F: Fn()>(f: F) {
    f();
}

fn main() {
    let x = 7;
    let print = || println!("{}", x);

    apply(print);
}
