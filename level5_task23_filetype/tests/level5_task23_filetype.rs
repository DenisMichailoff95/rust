use level5_task23_filetype::*;

#[test]
fn test_print_info() {
    let ft = FileType::Text(String::from("hello"));
    print_info(&ft);
}
