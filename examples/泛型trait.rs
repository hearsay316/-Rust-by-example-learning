struct Empty;
struct Null;

trait DoubleDrop<T>{
    fn double_drop(self,_:T);
}

// impl<T> DoubleDrop<T> for Empty{
//     fn double_drop(self, _: T) {
//     }
// }
// impl<T> DoubleDrop<T> for Null{
//     fn double_drop(self, _: T) {
//     }
// }
impl<T,U> DoubleDrop<T> for U{
    fn double_drop(self,_:T){}
}
fn main(){
    let empty = Empty;
    let null = Null;
    empty.double_drop(null);
    let empty = Empty;
    let null = Null;
    null.double_drop(empty);
  let a = "aaa";
    let b = "xxx";
    a.double_drop(b);
}