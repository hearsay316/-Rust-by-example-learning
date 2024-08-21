fn main() {

    let _x = Box::new(0i32);
    division(3,0);
    println!("This pint won't be reached!");
}
fn division(dividend:i32,divisor:i32)->i32{
    if divisor ==0{
        panic!("division by zero")
    }
    else {
        dividend/divisor
    }
}