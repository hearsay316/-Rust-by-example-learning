fn main() {
    let my_num: i32 = 10;
    let mu_num_ptr: *const i32 = &my_num;
    println!("{:?}", mu_num_ptr);
    let mut my_speed_ptr: i32 = 88;
    let my_speed_ptr: *mut i32 = &mut my_speed_ptr;
    println!("{:?}", my_speed_ptr);
    //  需要被封装成一个函数 ,返回排序后的数组
    fn sort_arr(mut arr: [i32; 5]) -> [i32; 5] {
        arr.sort_by(|a, b| b.cmp(a));
        arr
    }
    let arr = sort_arr([5, 3, 8, 1, 2]);
    println!("{:?}", arr);
}

// Invalid assistant message: content or tool_calls must be set
