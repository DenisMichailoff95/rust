use level9_task41_csv::*;

#[test]
fn test_parse_csv() {
    let csv = "name,age,city\nAlice,30,Paris\nBob,25,Lyon";
    let people = parse_csv(csv);
    assert_eq!(people.len(), 2);
    assert_eq!(people[0].name, "Alice");
    assert_eq!(people[0].age, 30);
    assert_eq!(people[0].city, "Paris");
    assert_eq!(people[1].name, "Bob");
    assert_eq!(people[1].age, 25);
    assert_eq!(people[1].city, "Lyon");
}

#[test]
fn test_parse_csv_invalid_row() {
    let csv = "name,age,city\nAlice,30,Paris\ninvalid\nBob,25,Lyon";
    let people = parse_csv(csv);
    assert_eq!(people.len(), 2);
}

#[test]
fn test_parse_csv_empty() {
    let csv = "name,age,city\n";
    let people = parse_csv(csv);
    assert!(people.is_empty());
}
