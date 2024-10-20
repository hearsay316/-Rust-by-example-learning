fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();
    fn_plain();
    fn_mut();
    fn_once();
    let add = returns_closure(1);
    let mun = add(6);
    println!("{}", mun);
    let add_2 = returns_closure2(1);
    let num = add_2(6);
    println!("{}", num);
}

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a: {}", text)
}
fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a: {}", text)
}
fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("This is a: {}", text)
}
fn returns_closure(num: i32) -> impl Fn(i32) -> i32 {
    move |x| x + 1 + num
}
fn returns_closure2(num: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + 1 + num)
}
