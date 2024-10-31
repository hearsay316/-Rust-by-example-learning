use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::thread;
use lazy_static::lazy_static;

lazy_static! {

    static ref USERS: Arc<Mutex<HashMap<String, User>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref COOKIES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref USED_INVITATION_CODES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}
#[derive(Debug)]
struct User {
    username: String,
    password: String, // In a real application, store hashed passwords!
    email: String,
    is_admin: bool,
}
fn main() {
    println!("Hello, world!");
    // let admin_user = User {
    //     username: "admin".to_string(),
    //     password: "ADMIN_PASSWORD".to_string(),
    //     email: "admin@example.com".to_string(),
    //     is_admin: true,
    // };


    let mut handles = vec![];
    for i in 0..10 {

        let handle = thread::spawn(move || {
            let admin_user = User {
                username: "admin".to_string(),
                password: "ADMIN_PASSWORD".to_string(),
                email: "admin@example.com".to_string(),
                is_admin: true,
            };
            let key = format!("admin{}", i);
            let mut users = USERS.lock().unwrap();
            users.insert(key, admin_user);
            println!("{:?}", users);
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    let mut users = USERS.lock().unwrap();
    // users.insert("admin".to_string(), admin_user);
    println!("{:?}", users);
    println!("{:?}", users.len());
    {
        users
    };
    get_user("admin");
    // println!("{:?}", users);
}

fn get_user(p0: &str) {
    println!("啊啊啊get_user :{}",p0);

    let mut users = USERS.lock().expect("xsxsxs");
    println!("啊啊啊:{:?}",users);
    // let user = users.get(p0).expect("sss");
    // println!("啊啊啊");
}