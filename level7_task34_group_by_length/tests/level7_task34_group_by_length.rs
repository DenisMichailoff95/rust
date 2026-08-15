use level7_task34_group_by_length::*;

#[test]
fn test_group_by_length() {
    let strings = vec![
        String::from("hi"),
        String::from("hello"),
        String::from("rust"),
        String::from("ok"),
    ];
    let groups = group_by_length(&strings);
    assert_eq!(groups.get(&2), Some(&vec!["hi".to_string(), "ok".to_string()]));
    assert_eq!(groups.get(&4), Some(&vec!["rust".to_string()]));
    assert_eq!(groups.get(&5), Some(&vec!["hello".to_string()]));
}
