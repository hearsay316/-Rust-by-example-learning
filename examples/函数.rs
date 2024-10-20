fn main() {
    fizzbuzz_to(100)
}
fn is_divisoble(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    println!("lhs% rhs is {}", lhs % rhs);
    lhs % rhs == 0
}
fn fizzbuzz(n: u32) {
    if is_divisoble(n, 15) {
        println!("fizzbuzz");
    } else if is_divisoble(n, 3) {
        println!("fizz");
    } else if is_divisoble(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n)
    }
}
