struct Person {
    name: String,
    age: u32,
}

fn to_json(p: &Person) -> String {
    let escaped_name = p.name.replace('"', "\\\"");
    format!(r#"{{"name":"{}","age":{}}}"#, escaped_name, p.age)
}

fn main() {
    let p = Person {
        name: String::from("Alice \"A\""),
        age: 30,
    };
    println!("{}", to_json(&p));
}
