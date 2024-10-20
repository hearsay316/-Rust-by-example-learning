static LANGUAGE: &'static str = "Rust";
const THESHOLD: i32 = 10;
fn is_big(n: i32) -> bool {
    n > THESHOLD
}

fn main() {
    let n = 16;
    println!("The is :{}", LANGUAGE);
    println!("The threshold is {}", THESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "samll" });

    // THESHOLD = 5;
}
