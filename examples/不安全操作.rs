use std::slice;

fn main() {
    let raw_p: *const u32 = &10;
    unsafe {
        assert!(*raw_p == 10);
    }

    let some_vertor = vec![1, 2, 3, 4];
    let pointer = some_vertor.as_ptr();
    let length = some_vertor.len();
    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);
        assert_eq!(some_vertor.as_slice(), my_slice)
    }
}
