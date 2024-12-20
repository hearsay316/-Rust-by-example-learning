use std::cmp::PartialOrd;

#[derive(Debug, PartialOrd, PartialEq)]
struct s {
    a: i32,
    b: String,
}

fn main() {
    let mut vec = vec![12, 23, 56];
    vec.sort_by(|a, b| a.cmp(&b));
    println!("{:?}", vec);
    // let f = 1.0f64;
    // let binding = f.to_string();
    // let v = binding.split('.').collect::<Vec<_>>();
    // let one = v[0].to_string();
    // let value = v[1].to_string();
    // println!("{:?}", value.len());
    // let a = (0..value.len())
    //     .enumerate()
    //     .fold(1, |acc, (index, _)| acc * 10);
    // println!(
    //     "{:?}  ",
    //     (one.parse::<f64>().unwrap() * (a as f64)) + value.parse::<f64>().unwrap()
    // );
}
