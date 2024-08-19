macro_rules! calculate{
    (eval $e:expr)=>{{

        let val:usize = $e;// 强制转换类型
        println!("{} = {}",stringify!($e),val);
    }}
}


fn main(){
    calculate!{
        eval 1 + 1 // 看到了吧 `eval` 可不是rust的关键字

    }
    calculate!{
        eval  (1+2)*(3/4)
    }
    add();
}

fn add(){
    println!("func: add")
}