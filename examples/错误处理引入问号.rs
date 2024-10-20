use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;
    Ok(first_number * second_number)
}
fn multiply_2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = r#try!(first_number_str.parse::<i32>());
    let second_number = r#try!(second_number_str.parse::<i32>());
    Ok(first_number * second_number)
}
fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error is {}", e),
    }
}
fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
    print(multiply_2("10", "2"));
    print(multiply_2("ds", "2"));
}
//rust 2018中 可以使用 try!  宏
// rust 2021之后 可以使用?  或者 r#try! 宏
