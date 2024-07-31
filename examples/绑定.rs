fn age()->u32{
    20
}

fn some_number()->Option<u32>{
    Some(42)
}
 fn main(){
     println!(" Tell me what type of person you  are");
     match age() {
         0                    =>println!("I haven't celebrated my first birthday yet"),
             n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
             n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
         n=>{
             println!("I'm an old person of age {:?}", n)
         }
     }
     // match age() {
     //     0             => println!("I haven't celebrated my first birthday yet"),
     //     // 可以直接匹配（`match`） 1 ..= 12，但那样的话孩子会是几岁？
     //     // 相反，在 1 ..= 12 分支中绑定匹配值到 `n` 。现在年龄就可以读取了。
     //     n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
     //     n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
     //     // 不符合上面的范围。返回结果。
     //     n             => println!("I'm an old person of age {:?}", n),
     // }

     match  some_number() {
         Some( n @ 43) =>println!("The Answer43:{}",n),
         Some( n @ 42) =>println!("The Answer:{}",n),
         Some(n)=>println!("Not interesing...{}",n),
         _ =>()
     }
 }