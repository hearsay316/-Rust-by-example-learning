mod my{
    // 一个公有的结构体，带有一个公有的字段（类型为泛型 `T`）
    pub struct OpenBox<T>{
        pub contents:T,
    }
    // 一个公有的结构体，带有一个私有的字段（类型为泛型 `T`）
    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct ClosedBox<T>{
        contents:T
    }
    impl<T> ClosedBox<T>{
        pub fn new(contents:T)->ClosedBox<T>{
            // 一个公有的构造器方法
            ClosedBox{
                contents
            }
        }
    }
}

fn main(){
    let open_box = my::OpenBox{contents:"public information"};
    println!("The open box contains:{}",open_box.contents);
    // 并且字段可以被访问
    let _close_box = my::ClosedBox::new("classiffied information");
    println!("The closed box contains :{:?}",_close_box);
}