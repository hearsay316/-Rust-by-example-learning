fn main() {
    let i = 3;
    {
        let borrow1 = &1;
        println!("borrow1:{}", borrow1);
    }
    {
        let borrow2 = &i;
        println!("borrow2 :{}", borrow2);
    }
}
