use std::num::ParseIntError;

// ^ expected `()`, found `Result<i32, ParseIntError>`
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("e is {}", e),
    }
}
fn main() {
    // let i:() = "t".parse::<i32>();
    let twenty = multiply("10", "2");
    print(twenty);
    let tt = multiply("t", "2");
    print(tt);
    let twenty_2 = multiply2("10", "2");
    print(twenty_2);
    let tt_2 = multiply2("t", "2");
    print(tt_2);
}
fn multiply2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}
