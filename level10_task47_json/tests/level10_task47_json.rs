use level10_task47_json::*;

#[test]
fn test_to_json() {
    let p = Person { name: String::from("Alice"), age: 30 };
    assert_eq!(to_json(&p), r#"{"name":"Alice","age":30}"#);
}

#[test]
fn test_to_json_special_chars() {
    let p = Person { name: String::from("Alice \"A\""), age: 30 };
    assert_eq!(to_json(&p), r#"{"name":"Alice \"A\"","age":30}"#);
}
