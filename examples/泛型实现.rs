struct S;
struct GenericVal<T>(T);
// GenericVal  的impl ,  此处我们显示的指定了类型参数;
impl GenericVal<f32> {} //指定为`f32`类型
                        // impl的泛型要在 impl 后面写<>  不是函数 在函数 名后面写
impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}
struct GenVal<T> {
    gen_val: T,
}
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}
// Gen_val的`impl` ,指定了T 是泛型类型
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}
fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    println!("x.value: {},y.value: {}", x.value(), y.value());
}
