Rust Library for Tracing errors to Telegram Bot user_id
=========================

A library for sending all tracing errors to your telegram bot user_id in private chat

## Example
Here is a simplest usage:

``` rust
use std::env;
use tracing::Level;

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let user_id = env::var("TELEGRAM_USER_ID").expect("TELEGRAM_USER_ID not set").parse::<i64>().expect("TELEGRAM_USER_ID must be i64");

    tracing_tg_bot_subscriber::new()
        .set_token(token)
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
    // TELEGRAM_BOT_TOKEN env variable in use
    // TELEGRAM_USER_ID env variable in use
    // Default bot tracing level is Level::ERROR
    // Default debug tracing level is Level::WARN
    tracing_tg_bot_subscriber::new()
        .register();

    Ok(())
}