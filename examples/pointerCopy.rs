fn main() {
    let my_num: i32 = 10;
    let mu_num_ptr: *const i32 = &my_num;
    println!("{:?}", mu_num_ptr);
    let mut my_speed_ptr: i32 = 88;
    let my_speed_ptr: *mut i32 = &mut my_speed_ptr;
    println!("{:?}", my_speed_ptr);
}
