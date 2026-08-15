use level3_task12_mutate::*;

#[test]
fn test_add_exclamation() {
    let mut s = String::from("hello");
    add_exclamation(&mut s);
    assert_eq!(s, "hello!");
}
