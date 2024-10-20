struct A;
struct S(A);
struct SGen<T>(T);
// 结构体也是类型
fn reg_fn(_s: S) {}
//SGen<A>是泛型  函数不是泛型
fn gen_spec_t(_s: SGen<A>) {}
//SGen<i32>是泛型  函数不是泛型
fn gen_spec_i32(_s: SGen<i32>) {}
//   函数是泛型 T  参数也是泛型
fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));
    generic::<char>(SGen('a'));
    generic(SGen('c'));
}
