use level9_task43_password::*;

#[test]
fn test_generate_password_length() {
    let pw = generate_password(8);
    assert_eq!(pw.len(), 8);
}

#[test]
fn test_generate_password_deterministic() {
    let pw1 = generate_password(8);
    let pw2 = generate_password(8);
    assert_eq!(pw1, pw2);
}
