fn main() {
    let mut array: [i32; 3] = [0; 3];

    array[1] = 1;
    array[2] = 2;

    assert_eq!([1, 2], &array[1..]);

    // 该循环打印: 0 1 2
    for x in array {
        println!("{x} ");
    }
}
