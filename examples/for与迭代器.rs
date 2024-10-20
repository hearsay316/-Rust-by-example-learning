fn main() {
    let names = vec!["box", "Frank", "ferris"];
    for name in names.iter() {
        // println!("{}",*name);
        // println!()
        match name {
            &"box" => println!("这个是一个小游戏"),
            _ => println!("hello {}", name),
        }
    }
    // into_iter()//  会消耗集合,在每次迭代中,集合数据本身会被提供. 一旦集合被消耗了, 之后就无法在使用了,以为他竟在循环中被 "移除" move 了
    for name in names.into_iter() {
        println!("{}", name);
    }
    let mut names = ["box", "Frank", "Ferris"];
    for name in names.iter_mut() {
        if (*name == "box") {
            *name = "box2"
        }
    }
    println!("{:?}", names);
}
