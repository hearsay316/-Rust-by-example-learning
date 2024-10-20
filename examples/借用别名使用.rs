#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };
    let borrowed_point = &point;
    let another_point = &point;

    //数据可以通过引用类型来访问
    println!(
        " Point has coordinates :({},{},{})",
        borrowed_point.x, another_point.y, point.z
    );
    // let mutable_borrow = &mut point;
    println!(
        " Point has coordinates :({},{},{})",
        borrowed_point.x, another_point.y, point.z
    );
    // 不可变引用不在用于其他代码,因此可以使用可变引用的重新借用

    let mutable_borrow = &mut point;
    mutable_borrow.z = 1;
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    let y = &mutable_borrow.y;
    //  println!("Point z coordinate is {}",point.z);
    println!(
        "Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );
    let new_borrowed_point = &point;
    println!(
        "point now has coordinate is ({},{}, {})",
        new_borrowed_point.z, new_borrowed_point.x, new_borrowed_point.y
    );
}
