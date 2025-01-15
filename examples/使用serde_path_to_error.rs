use serde::Deserialize;
use std::collections::BTreeMap as Map;

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    dependencies: Map<String, Dependency>,
}

#[derive(Deserialize, Debug)]
struct Dependency {
    version: String,
}

fn main() {
    let j = r#"{
        "name": "demo",
        "dependencies": {
            "serde": {
                "version": "1"
            }
        }
    }"#;

    // Some Deserializer.
    let jd = &mut serde_json::Deserializer::from_str(j);

    let result: Result<Package, _> = serde_path_to_error::deserialize(jd);
    match result {
        Ok(value) => println!("{:?}", value),
        Err(err) => {
            println!("{}", err);
            let path = err.path().to_string();
            println!("{}", path);
            assert_eq!(path, "dependencies.serde.version");
        }
    }
}
