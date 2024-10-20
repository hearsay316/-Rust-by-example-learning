struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}
impl Contains<i32, i32> for Container {
    // 如果存储的数字和给定的数字相等则为真.
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        println!("{}-{}-{}-{}", number_1, number_2, self.0, self.1);
        (&self.0 == number_1) == (&self.1 == number_2)
    }
    // 得到第一个数字
    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
}
//  容器`C`就包括了`A` `B`类型, 鉴于此,必须指出`A` 和`B` 显得很麻烦
fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
    container.last() - container.first()
}

fn main() {
    let number_1 = 1;
    let number_2 = 10;
    let container = Container(number_1, number_2);
    println!(
        "Doe|s container contain {} and {} :{}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number:{}", container.first());
    println!("Last number:{}", container.last());
    println!("The difference is:{} ", difference(&container));
}
