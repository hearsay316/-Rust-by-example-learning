use std::fmt;

struct List(Vec<i32>);
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let list_vec = &self.0;
        write!(f, "{{")?;
        for (index, value) in list_vec.iter().enumerate() {
            if index > 0 && index % 2 == 0 {
                write!(f, ", ")?;
            }
            if index > 0 && index % 2 != 0 {
                write!(f, ": ")?;
            }
            // if index == 0 {
            //     write!(f, "{}", value)?;
            //     continue;
            // }
            // if index % 2 == 0 {
            //     write!(f, ", ")?;
            // } else {
            //     write!(f, ": ")?;
            // }
            write!(f, "{}", value)?;
        }
        write!(f, "}}")
    }
}

fn main() {
    let list_vec = List(vec![0, 1, 1, 2, 2, 3]);
    println!("{}", list_vec);
}
// test  rust定义变量 let  变量 不能和 struct 同名
