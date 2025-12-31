use aura_bot::config;
use std::env;

fn with_env_var<T, F: FnOnce() -> T>(key: &str, val: Option<&str>, f: F) -> T {
    let prev = env::var(key).ok();
    match val {
        Some(v) => unsafe { env::set_var(key, v) },
        None => unsafe { env::remove_var(key) },
    }
    let res = f();
    match prev {
        Some(v) => unsafe { env::set_var(key, v) },
        None => unsafe { env::remove_var(key) },
    }
    res
}

#[test]
fn message_chance_default() {
    with_env_var("MESSAGE_CHANCE", None, || {
        assert!((config::message_chance_from_env() - 0.03f32).abs() < f32::EPSILON);
    });
}

#[test]
fn message_chance_parses() {
    with_env_var("MESSAGE_CHANCE", Some("0.5"), || {
        assert!((config::message_chance_from_env() - 0.5f32).abs() < f32::EPSILON);
    });
}

#[test]
fn bot_token_result_ok() {
    with_env_var("BOT_TOKEN", Some("x-token"), || {
        assert!(config::bot_token_from_env().is_ok());
    });
}

#[test]
fn bot_token_result_missing() {
    with_env_var("BOT_TOKEN", None, || {
        assert!(config::bot_token_from_env().is_err());
    });
}
