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

    // let a_iter = a.iter().cycle().count();
    // println!("{:?}", a_iter);
    // a_iter.for_each(|x| println!("{}", x));
    println!("这个是测试");
    let sum_vec = [Some(1), Some(2), Some(3)];
    let sum = sum_vec.into_iter().sum::<Option<i32>>();
    println!("{:?}", sum);
    println!("{:?}", sum_vec);
    let v = ["1".to_string(), "2".to_string(), "3".to_string()];
    let mut iter = v.iter();
    println!("{:?}", v);
    println!("{:?}", iter.count());

    use std::num::NonZeroUsize;
    let a = [1, 2, 3, 4];
    let mut iter = a.iter();

    assert_eq!(iter.advance_by(2), Ok(()));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.advance_by(0), Ok(()));
    assert_eq!(iter.advance_by(100), Err(NonZeroUsize::new(99).unwrap())); // 仅跳过 `&4`
}
