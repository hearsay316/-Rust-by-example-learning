// pub trait Iterator {
//  // 被迭代的类型。
//     type Item;
//     fn any<F>(&self, f: F) -> bool
//     where
//         F: Fn(&Self::Item) -> bool,
//     {
//         f(&&self)
//     }
// }
//any() 接受一个返回 true 或 false 的闭包。
// 它将这个闭包应用于迭代器的每个元素，如果它们中的任何一个返回 true，
// 那么 any() 也是如此。 如果它们都返回 false，则返回 false。
fn main(){
    let vec1 = vec![1,2,3];
    let vec2 = vec![4,5,6,];
    println!("2 in vec1:{}",vec1.iter().any(|&x|x==2));
    println!("2 in vec2:{}",vec2.iter().any(|&x|x==2));


    let array1 = [1,2,3];
    let array2 = [4,5,6];
    println!("2 in array1 :{}",array1.iter().any(|&x|2==x));
    println!("2 in array2 :{}",array2.iter().any(|&x|x==2));
}