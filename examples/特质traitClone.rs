#[derive(Debug, Clone, Copy)]
struct Nil;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);
fn main() {
    let nil = Nil;
    let copied_nil = nil; //  复制
    println!("original :{:?}", nil);
    println!("copy :{:?}", copied_nil);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("orgianl:{:?}", pair);
    let moved_pair = pair;
    println!("copy :{:?}", moved_pair);
    // println!("path :{:?}",pair);
    let clone_pair = moved_pair.clone();
    drop(moved_pair);

    println!("cone:{:?}", clone_pair);
}
