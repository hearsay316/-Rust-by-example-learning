fn main() {
    let s = "hhh";
    let s1 = s.clone();
    let s2 = s;
    let s3 = &s[0..2];
    let s4 = s.to_string();
    println!("{:p}--{}", s1, s1);
    println!("{:p}--{}", s2, s2);
    println!("{:p}--{}", s3, s3);
    println!("{:p}--{}", s, s);
    println!("{:p}--{}", &s4 as *const _, s.to_string());
}
