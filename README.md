Rust Library for Tracing errors to Telegram Bot user_id
=========================

A library for sending all tracing errors to your telegram bot user_id in private chat

## Example
Here is simplest usage:

``` rust
use std::env;
use tracing::Level;

fn main() {
    let api = Api::new(config.bot_key());
    let user_id = env::var("TELEGRAM_USER_ID").expect("TELEGRAM_USER_ID not set").parse::<i64>().expect("TELEGRAM_USER_ID must be i64");

    tracing_tg_bot_subscriber::new(api.clone())
        .set_user_id(user_id)
        .set_bot_level(Level::WARN)
        .set_debug_level(Level::INFO)
        .register();

    Ok(())
}
```

You can rely on the library to recognize data from environment variables on its own:

``` rust
use std::env;
use tracing::Level;

fn main() {
    // TELEGRAM_USER_ID env variable in use
    // Default bot tracing level is Level::ERROR
    // Default debug tracing level is Level::WARN
    let api = Api::new(config.bot_key());
    tracing_tg_bot_subscriber::new(api.clone())
        .register();

    Ok(())
}