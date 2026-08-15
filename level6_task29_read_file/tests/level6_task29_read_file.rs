use level6_task29_read_file::*;

#[test]
fn test_read_file_to_string() {
    let result = read_file_to_string("nonexistent.txt");
    assert!(result.is_err());
}
