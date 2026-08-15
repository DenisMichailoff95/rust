use level7_task35_union::*;

#[test]
fn test_intersection() {
    let mut result = intersection(&[1, 2, 3], &[2, 3, 4]);
    result.sort();
    assert_eq!(result, vec![2, 3]);
    assert!(intersection(&[], &[1, 2]).is_empty());
}
