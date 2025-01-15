use serde::Deserialize;
use serde_json::Deserializer;

#[derive(Deserialize, Debug)]
struct MyStruct {
    field: String,
}

fn main() {
    let bytes = br#"{"field": "value"}"#; // 字节切片
    println!("bytes :{:?}", bytes);
    let mut deserializer = Deserializer::from_slice(bytes);
    // println!("{:?}", deserializer);
    let result: MyStruct = serde::de::Deserialize::deserialize(&mut deserializer).unwrap();
    println!("{:?}", result); // 输出: value
}
