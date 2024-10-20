/*
`A`和`B`在trait 里面通过`type`关键字来定义.
(注意:此时的`type`不同于类型取别名时的`type`)
trait Contains{
type:A;
type:B;
fn contains(&self,_:&Self::A,_:Self::B)->bool;

}


fn difference<A,B,C> (conainer:&C)->i32 where
c:Contains<A,B>{}

// 可以使用关联类型
 fn difference<C:contains>(container:&C)->i32{..}
*/
// 让我们使用关联类型来重写上一节的例子
struct Container(i32, i32);
/*
这个trait 检查给定的2个项是否储存于容器之中
并且能够获得容器的第一个或最后一个值
 */
trait Contains {
    type AA;
    type B;
    fn contains(&self, _: &Self::AA, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}
impl Contains for Container {
    type AA = i32;
    type B = i32;
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
}
fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;
    let container = Container(number_1, number_2);
    println!(
        "Does container contain {} and {} :{}",
        &number_1,
        &number_2,
        &container.contains(&number_1, &number_2)
    );
    println!("First number :{}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is:{}", difference(&container));
}
