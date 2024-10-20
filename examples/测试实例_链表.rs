use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}
impl List {
    fn new() -> List {
        Nil
    }
    fn preend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }
    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{},{}", head, tail.stringify())
            }
            Nil => "Nil".to_string(),
        }
    }
}
fn main() {
    let mut list = List::new();
    list = list.preend(1);
    list = list.preend(2);
    list = list.preend(3);
    println!("linked list has length:{}", list.len());
    println!("{}", list.stringify());
}
