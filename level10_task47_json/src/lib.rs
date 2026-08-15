#![allow(dead_code)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

pub fn to_json(p: &Person) -> String {
    let escaped_name = p.name.replace('"', "\\\"");
    format!(r#"{{"name":"{}","age":{}}}"#, escaped_name, p.age)
}
