// 解引用 使用 *
// 解构使用 & , ref  和  ref mut

fn main(){
    let reference = &4;
    match reference {
        val =>{
            println!("Got a value via destructuring: {:?}",val);
        }
    }

    match *reference{
        val=>println!("Got a value via dereferencing: {:?}",val)
    }

    let _not_a_reference = 3;
    //   得到一个引用
    let ref _is_a_reference = 3;
     // 相应的,定义两个非引用的变量,通过 ref  或 ref mut 仍可以获取其引用
    let  value  = 5;
    let  mut mut_value = 6;
    println!("{}",mut_value);
    // 使用 `ref` 关键字来创建引用
    // 下面的r 是 `&i32`类型,它像`i32`一样可以直接打印, 因此用法上
    // 似乎看不出什么区别,  但是读者可以把``println!`里就不是`*val`,因为不能对整数解引用
    match value {
        ref r =>println!("Got 啊 :{:?}",*r)
    }
  match mut_value {
      ref mut m=>{
          *m+10;
          println!("We add 10. `nut_value` :{:?} mut_value:{}",m,mut_value);
          // println!("{}",mut_value)
      }
  }
}