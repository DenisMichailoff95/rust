use level1_task5_array_processing::*;

#[test]
fn test_array_constants() {
    assert_eq!(ARRAY.len(), 10);
    assert_eq!(ARRAY[0], 4);
    assert_eq!(ARRAY[9], 0);
}
