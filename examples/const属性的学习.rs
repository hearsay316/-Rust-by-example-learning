struct SS {
    x: i32,
}

const S: SS = SS { x: 42 };
fn main() {
    let mut v = S;
    v.x += 1;
    v.x += 1;
    v.x += 1;
    S.x += 1;
    print!("{}{}", v.x, S.x);
}
