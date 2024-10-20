#[derive(PartialEq)]
struct Years(i64);
struct Days(i64);
const NUM: i64 = 365;
impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * NUM)
    }
}
impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / NUM)
    }
}
//成年num
fn old_enough(age: &Years) -> bool {
    age.0 > 18
}
fn main() {
    let age = Years(5);
    let age_days = age.to_days();
    let days = Days(1000);
    let days_age = days.to_years();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    println!("Old enough {}", days.to_years() == days_age);
}
