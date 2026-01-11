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
fn bot_token_result_default() {
    with_env_var("BOT_TOKEN", Some("x-token"), || {
        with_env_var("MESSAGE_CHANCE", Some("0.420"), || {
            // Call from_env
            let config = config::Config::from_env();

            // Assert values
            assert_eq!(config.token, "x-token");
            assert!((config.message_chance - 0.420).abs() < f32::EPSILON);
        });
    });
}
