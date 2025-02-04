// pub trait Iterator {
//     type Item;
//     fn find<P> (&mut self ,predicate:P)->Option<Self::Item> where p:FnMut(&Self::Item)->bool{
//
//     }
// }

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let mut iter = vec1.iter();

    let mut into_iter = vec2.into_iter();
    println!("Find 2 in vec1:{:?}", iter.find(|&&x| x == 2).unwrap());
    println!("Find 2 in vec2:{:?}", into_iter.find(|&x| x == 2));
    // println!("{:?}",vec2.into_iter());
    let array1 = ["1", "2", "3"];
    let array2 = ["44", "54", "26"];
    println!("Find 2 in array1:{:?}", array1.iter().find(|&x| *x == "2"));
    println!(
        "Find 2 in array2:{:?}",
        array2.into_iter().find(|&x| x == "2")
    );
    // println!("{}",(array2.into_iter()) == (array2.into_iter()));
}
