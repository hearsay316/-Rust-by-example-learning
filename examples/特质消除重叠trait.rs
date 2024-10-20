trait UsernameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self) -> u8;
}
struct Form {
    username: String,
    age: u8,
}
impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}
impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        username: "rustacean".to_string(),
        age: 28,
    };
    // println!("{}",form.get());

    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_string(), username);
    let age = form.get();
    assert_eq!(28, age);
}
