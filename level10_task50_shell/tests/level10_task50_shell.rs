use level10_task50_shell::*;

#[test]
fn test_parse_cd() {
    assert_eq!(parse("cd /home"), Some(Command::Cd("/home".to_string())));
}

#[test]
fn test_parse_pwd() {
    assert_eq!(parse("pwd"), Some(Command::Pwd));
}

#[test]
fn test_parse_ls() {
    assert_eq!(parse("ls"), Some(Command::Ls));
}

#[test]
fn test_parse_history() {
    assert_eq!(parse("history"), Some(Command::History));
}

#[test]
fn test_parse_exit() {
    assert_eq!(parse("exit"), Some(Command::Exit));
}

#[test]
fn test_parse_unknown() {
    assert_eq!(parse("unknown"), None);
    assert_eq!(parse(""), None);
}

#[test]
fn test_parse_cd_no_path() {
    assert_eq!(parse("cd"), None);
}
