/*pub trait Add<RHS = Self>{
    type  Output;
    fn add(self,rhs:RHS)->Self::Output;
}
impl<U> Add for T<U>{
    type Output = T<U>;
    ...
}
*/
use std::marker::PhantomData;
use std::ops::Add;

// 创建空枚举来表示单位
#[derive(Debug,Clone,Copy)]
enum Inch{}

#[derive(Debug,Clone,Copy)]
enum Mm{}
/// `Length`是表示带有虚7类型参数`Unit`的类型.
/// 而且对于表示长度的类型(即`f64`)而言,`Length`不是泛型
///
/// `f64` 已经实现了`Cone`和`Copy` trait
#[derive(Debug,Clone,Copy)]
struct Length<Unit>(f64,PhantomData<Unit>);

///`Add` trait 定义了 `+`运算符的行为
impl <Unit> Add for Length<Unit>{
    type Output = Length<Unit>;
    // add()返回一个含有和新的`Length`结构体
    fn add(self,rhs:Length<Unit>)->Length<Unit>{
        // `+` 调用了针对`f64`类型的`Add`的实现
        Length(self.0+rhs.0,PhantomData)
    }
}

fn  main(){
    let one_foot:Length<Inch> = Length(12.0,PhantomData);
    let one_meter:Length<Mm> = Length(1000.0,PhantomData);
    let two_feet = one_foot+ one_foot;
    let two_meters = one_meter+one_meter;
    println!("one_foot+ one_foot = {:?} in ",two_feet.0);
    println!("one_meter + one_meter = {:?} mm",two_meters);

    // println!("{:?}",one_foot+one_meter);
}