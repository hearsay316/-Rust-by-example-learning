fn mutiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}
fn main() {
    let first = 2; // 较长的生命周期
    {
        let second = 3; // 较短的生命周期
        println!("The product is {}", mutiply(&first, &second));
        println!("{} is the first ", choose_first(&first, &second));
    }
}