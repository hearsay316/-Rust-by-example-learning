fn main() {
    // let v1 = vec![1, 2, 3];
    // let v1_iter: (Vec<_>, Vec<_>) = v1.iter().cloned().unzip();
    // println!("{:?}", v1_iter);
    let a = [1, 2, 3];

    let v_cloned: Vec<_> = a.iter().cloned().map(|x| x).collect();

    // cloned is the same as .map(|&x| x), for integers
    let v_map: Vec<_> = a.iter().map(|&x| x.clone()).collect();
    // println!("{:?}", a==v_map);
    assert_eq!(v_cloned, vec![1, 2, 3]);
    assert_eq!(v_map, vec![1, 2, 3]);

    let a_iter = a.iter().cycle();
    a_iter.for_each(|x| println!("{}", x));
    println!("这个是测试")
}
