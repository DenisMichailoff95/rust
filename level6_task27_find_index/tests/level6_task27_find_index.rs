use level6_task27_find_index::*;

#[test]
fn test_find_index() {
    assert_eq!(find_index(&[1, 2, 3, 4], 3), Some(2));
    assert_eq!(find_index(&[1, 2, 3], 5), None);
    assert_eq!(find_index(&[], 1), None);
}
