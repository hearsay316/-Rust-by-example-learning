fn main() {
    let pair = (-2, 2);
    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // if 条件部分是一个卫语句
        (x, y) if x + y == 0 => println!(" Antimatter ,kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation ..."),
    }
}
