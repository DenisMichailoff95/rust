use level5_task24_command::*;

#[test]
fn test_parse_command() {
    assert_eq!(parse_command("quit"), Some(Command::Quit));
    assert_eq!(parse_command("add 1 2"), Some(Command::Add(1.0, 2.0)));
    assert_eq!(parse_command("sub 5 3"), Some(Command::Sub(5.0, 3.0)));
    assert_eq!(parse_command("unknown"), None);
}
