use level6_task30_env_var::*;

#[test]
fn test_get_env_var_or_default() {
    let result = get_env_var_or_default("PATH");
    assert!(result.is_ok());
}

#[test]
fn test_get_env_var_missing() {
    let result = get_env_var_or_default("NONEXISTENT_VAR_12345");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Env var 'NONEXISTENT_VAR_12345' not set");
}
