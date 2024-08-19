fn next_birthday(current_age:Option<u8>)->Option<String>{
    let next_age:u8 = current_age?;
    Some(format!("Next year I will be {}",next_age))
}

struct Person{
    job:Option<Job>
}

#[derive(Clone,Copy)]
struct Job{
    phone_number:Option<PhoneNumber>
}
#[derive(Clone,Copy)]
struct PhoneNumber{
    area_code:Option<u8>,
    number:u32
}
impl Person {
    fn work_phone_are_code(&self)->Option<u8>{
        self.job?.phone_number?.area_code
    }
}
fn main(){
    let age = Some(32);
    let age_string = next_birthday(age);
    println!("{:?}",age_string);
    let p = Person{
        job:Some(Job{
            phone_number:Some(
                PhoneNumber{
                    area_code:Some(61),
                    number:43922222
                }
            )
        })
    };
    assert_eq!(p.work_phone_are_code(),Some(61));
}