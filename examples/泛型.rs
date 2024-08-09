struct A;
// 具体类型
struct Single(A);
//因为使用了泛型参数, 所以 SingleGen 是泛型类型
struct SingleGen<T>(T);

fn main(){
    // Single是具体类型,并且显示地使用类型`A`
    let _s = Single(A);
    let _char:SingleGen<char> =SingleGen('a');


    // 也可以用类型隐式推导指定
    let _t = SingleGen("a");
    let _char = SingleGen(A);
    let _i32 = SingleGen(6);
    let _str = SingleGen("可能");
}


