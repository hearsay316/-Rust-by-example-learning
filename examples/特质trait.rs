struct Sheep {
    naked:bool,
    name:&'static str
}
trait Animal {
    //静态方法签名:"self` 表示实现者类型
    fn new (name:&'static str)->Self;
    // 实例方法签名 : 这些方法将返回一个字符串.
    fn name(&self) ->&'static str;
    fn noise(&self)->&'static str;
    // trait 可以提供默认方法
    fn talk (&self){
        println!("{} says {}",self.name(),self.noise());
    }
}
impl Sheep{
    fn  is_naked(&self)->bool{
        self.naked
    }
    fn shear(&mut self){
        if self.is_naked(){
            println!("{} is already naked...",self.name())
        }else{
            println!("{} gets a haircut!",self.name);
            self.naked = true;
        }
    }
}
// 对Sheep 实现 Animal trait
impl Animal for Sheep{
    fn new(name:&'static str)->Sheep{
        Self{
            name:name,naked:false
        }
    }
    fn name(&self)->&'static str {
        self.name
    }
    fn noise(&self)->&'static str{
        if self.is_naked(){
            "baaaaah?"
        }else {
            "baaaaaah!"
        }
    }
    fn talk(&self){
        println!("{} pauses briefly..{}",self.name, self.noise());
    }
}
fn main(){
    let mut dolly:Sheep = Animal::new("Dolly");
    dolly.talk();
    dolly.shear();
    dolly.talk();
}