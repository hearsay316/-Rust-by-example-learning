use std::fmt;
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{} + {}i",self.real,self.imag)
    }
}
impl fmt::Debug for Complex {
    // fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
    //     f.debug_struct("Complex").field("real",&self.real).field("imag",&self.imag).finish()
    // }
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        write!(f,"Complex {{ real: {} ,imag: {} }}",self.real,self.imag)
    }
}
fn main(){
    let item = Complex{real:3.3,imag:7.2};
    println!("Display:{}",item);
    println!("debug:{:?}",item);
}