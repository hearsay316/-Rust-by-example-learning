use std::ptr;

fn main() {
    let five = 5;
    let other_five = 5;
    let five_ref = &five;
    let same_five_ref = &five;
    let other_five_ref = &other_five;
    assert!(five_ref == same_five_ref);
    assert!(five_ref == other_five_ref);
    assert!(ptr::eq(five_ref, same_five_ref));
    assert!(!ptr::eq(five_ref, other_five_ref));
    let arr = [1.1, 2.2, 3.3];
    let arr2 = arr;
    assert_eq!(arr, arr2);
}
