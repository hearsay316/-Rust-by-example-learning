// rust中的闭包(closure),也叫做lambda表达式 ,是一类能够捕获周围作用域中变量的函数.fn main() {
// 例如,一个可以捕获x变量的闭包如下
/*
|val| val+ x
*/
fn  main(){

    fn function (i:i32)->i32{i+1}
    // 闭包是匿名的,这个我们将和他们绑定到引用
    // 类型标注和函数的一样  ,不过类型标注和使用`{}`来围住函数体都是可选的.
    // 这些匿名函数(nameless function ) 被赋值给合适地命名的变量 .
    let closure_annotated = |i:i32|->i32{i+1};
    // 参数类型未定类型, 在调用时候约定
    let closure_inferred = |i| i+1;
    // 注: 将闭包绑定到以哦你用的说法可能不准
    // 闭包表达式产生的类型就是"闭包类型", 不属于引用类型,而且确实无法对上面两个`closure_xxx`变量解引用
    let i = 1;
    println!("function :{}",function(1));
    println!("closure_annotated:{}",closure_annotated(i));
    println!("closure_inferred :{}",closure_inferred(i));

    // 没有参数的闭包,返回一个`i32`类型
    // 返回类型是自动推导的
    let one = ||1;
    println!("closure returning one:{}",one());
}