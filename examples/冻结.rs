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