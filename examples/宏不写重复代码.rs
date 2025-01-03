use core::fmt::{Debug, Display};
use std::ops::{Add, Mul, Sub};
macro_rules! asser_equal_len {
    ($a:ident,$b:ident,$func:ident,$op:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?}: dimension mismatch :{:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),),
            stringify!($op),
            ($b.len(),)
        );
    };
}
macro_rules! op {
    ($func:ident, $bound:ident,$op:tt,$method:ident) => {
        fn $func<T: $bound<T, Output = T> + Copy + Debug + Display>(xs: &mut Vec<T>, ys: &Vec<T>) {
            asser_equal_len!(xs, ys, $func, $op);
            println!("xs:{:?},ys:{:?}", xs, ys);
            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
            }
        }
    };
}
op!(add_assign,Add,+=,add);
op!(mul_assign,Mul,*=,mul);
op!(sub_assign,Sub,-=,sub);
mod test {
    use std::iter;
    macro_rules! test {
        ($func:ident, $x:expr,$y:expr,$z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();
                    super::$func(&mut x, &y);
                    assert_eq!(x, z);
                }
            }
        };
    }
    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);
    #[test]
    fn user() {
        assert_eq!(4, 4);
    }
}

fn main() {}
