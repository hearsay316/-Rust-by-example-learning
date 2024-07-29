// 当数据被相同的名称不变地绑定是, 他还会冻结(freeze).在不可变绑定超出作用域之前,无法修改已经冻结的数据;
fn main(){
    let mut _mutable_integer = 1i32;
    {
        let _mutable_integer = _mutable_integer;
        println!("mut int: {}",_mutable_integer);
        // _mutable_integer = 10;
    }
    _mutable_integer = 10;
    println!("mut int : {}",_mutable_integer);
}
/*类型系统
Rust 提供了多种机制，用于改变或定义原生类型和用户定义类型。接下来会讲到：

原生类型的类型转换（cast）。
指定字面量的类型。
使用类型推断（type inference）。
给类型取别名（alias）。*/

/*


*/