fn creat_box() {
    let _box1 = Box::new(3i32);
    // _box1 在这里被销毁
}

fn main() {
    let _box2 = Box::new(5i32);
    {
        let _box3 = Box::new(4i32);
        // `_box3`在这里会被销毁
    }

    for _ in 0u32..1_000 {
        creat_box();
    }
    // _box 在这里会被销毁
    let x = ToDrop;
    println!("Made a ToDrop");
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("被销毁了");
    }
}
