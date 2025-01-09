fn main() {
    fn get_a_number() -> Option<u32> {
        None
    }
    loop {
        let num: u32 = match get_a_number() {
            Some(num) => num,
            None => break,
        };
    }
}
