use std::marker::PhantomData;

#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}
fn main() {
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('q', PhantomData);
    let _tuple2: PhantomTuple<char, f32> = PhantomTuple('q', PhantomData);
    let _struct1: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // 编译器错误
    println!("_tuple1==_tuple2 yields: {}", _tuple1 == _tuple2);
    println!("_struct1== _struct2 yields :{} ", _struct1 == _struct2);
}
