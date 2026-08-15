#![allow(dead_code)]
use std::env;

pub fn get_env_var_or_default(name: &str) -> Result<String, String> {
    env::var(name).map_err(|_| format!("Env var '{name}' not set"))
}
