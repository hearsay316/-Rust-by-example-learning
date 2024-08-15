#[derive(Clone,Copy)]
struct Point{ x:i32, y:i32}

fn main(){
    let c = 'Q';
    let ref ref_c1 = c;
    let ref_c2 = &c;
    println!("ref_c1 == ref_c2 :{}",*ref_c1==*ref_c2);
    let point = Point{x:1,y:1};
    let _copy_of_x = {
       let Point {x:ref ref_x,y:_} = point;
       ref_x
   };
    let _copy_of_x = &1;
    println!("_copy_of_x == _copy_of_x2 :{}",*_copy_of_x==*_copy_of_x);

    let mut mutable_point = point;
    println!("{}, {},",_copy_of_x,mutable_point.x);
    mutable_point.x = 4;
    println!("{},{}",_copy_of_x,mutable_point.x);
    {
        let Point {x:_,y:ref mut mut_ref_y} = mutable_point;
        *mut_ref_y = 3;
    }
    println!("point is ({},{})",point.x,point.y);
    println!("mutable_point is ({},{})",mutable_point.x,mutable_point.y);
    let mut mutable_tuple = (Box::new(5u32),3u32);
    println!("tuple is :{:?}",mutable_tuple);
    {
        let (_,ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    println!("tuple is {:?}",mutable_tuple);
}

/*
 --> examples\借用ref模式.rs:9:9
  |
9 |     let point = Point{x:1,y:1};
  |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_point`
  |
warning: fields `x` and `y` are never ready default
 --> examples\借用ref模式.rs:2:15
  |
2 | struct Point{ x:i32, y:i32}
  |        -----  ^      ^
  |        |
  |        fields in this struct
  |
  = note: `Point` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis
 fields `x` and `y` are never read
*/