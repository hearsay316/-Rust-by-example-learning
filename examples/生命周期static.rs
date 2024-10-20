static NUM: i32 = 18;
fn coer_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        let static_string = "I'm in read-only memory";
        println!("static_string:{}", static_string);
    }
    {
        let lifetime_num = 9;
        let coerced_static = coer_static(&lifetime_num);
        println!("coerced_static :{}", coerced_static);
    }
    println!("NUM:{} stays accessoble!", NUM);
}
