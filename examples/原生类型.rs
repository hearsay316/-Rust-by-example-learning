/*
rust提供了很多原生类型
标量类型 :
 有符号整数 :  i8 i16 i32  i64 isize
 无符号整数 : u8 u16  u31 u64 usize
 浮点数: f32  f64
 char :单个Unicode字符 如 'a' 每个都是四个字节
 bool 布尔类型 只能是true 或  false
 单元类型 ()   标量类型 一个6个
 复合类型  两个
 数组 元组




*/

fn  main(){
let  logical:bool = true;
    let a_float:f64 = 1.0;
    let an_integer = 5i32;
    let default_float = 3.0;
    let default_integer = 7;
    let mut inferred_type = 7;
    inferred_type = 429467296i64;

    let mut mutable = 12;
    mutable  = 21;
    // mutable = true;
    let mut matble = true;
   let mun =  single_number(vec![1,23,23,25,25]);
    println!("mun :{mun}")
}
//  力扣  136
fn single_number(nums: Vec<i32>) ->  i32 {
   let mut  map = std::collections::HashMap::new();
    for num in nums {
        if(map.contains_key(&num)){
            *map.get_mut(&num).unwrap() += 1;
        }
        else{
            map.insert(num, 1);
        }
    }
    for (key, val) in map.iter() {
        println!("key: {key} val: {val}");
        if *val == 1 {
            return *key;
        }
    }
    return  0;
}