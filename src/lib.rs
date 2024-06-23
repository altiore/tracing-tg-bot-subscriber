use crate::bot_writer::BotWriter;
use tracing::Metadata;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;
use tracing_subscriber::{
    filter::LevelFilter,
    fmt::{Layer, MakeWriter},
};

mod bot_writer;
mod config;

pub struct TracingTgBotSubscriber {
    token: Option<String>,
    user_id: Option<i64>,
    /// Level of bot tracing level
    bot_level: tracing::Level,
    /// Level of debug process tracing level
    debug_level: tracing::Level,
}

pub fn new() -> TracingTgBotSubscriber {
    TracingTgBotSubscriber::new()
}

impl TracingTgBotSubscriber {
    fn new() -> Self {
        TracingTgBotSubscriber {
            token: None,
            user_id: None,
            bot_level: tracing::Level::ERROR,
            debug_level: tracing::Level::WARN,
        }
    }

    pub fn set_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_owned());
        self
    }

    pub fn set_user_id(mut self, user_id: i64) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn set_bot_level(mut self, level: tracing::Level) -> Self {
        self.bot_level = level;
        self
    }

    pub fn set_debug_level(mut self, level: tracing::Level) -> Self {
        self.debug_level = level;
        self
    }

    pub fn register(mut self) {
        if let None = self.token {
            match config::get_var("TELEGRAM_BOT_TOKEN") {
                Ok(token) => {self.token = Some(token)}
                Err(err) => match err {
                    config::ConfigError::NotPresent(_) => panic!("Please set up TELEGRAM_BOT_TOKEN env variable or use .set_token method for that"),
                    config::ConfigError::NotUnicode(_, _) => panic!("Incorrect format of TELEGRAM_BOT_TOKEN env variable")
                }
            }
        }

        if let None = self.user_id {
            match config::get_var("TELEGRAM_USER_ID") {
                Ok(user_id) => match user_id.parse::<i64>() {
                    Ok(user_id) => {self.user_id = Some(user_id)},
                    Err(_) => panic!("Incorrect format for TELEGRAM_USER_ID variable. Must be i64 number")
                }
                Err(err) => match err {
                    config::ConfigError::NotPresent(_) => panic!("Please set up TELEGRAM_BOT_TOKEN env variable or use .set_token method for that"),
                    config::ConfigError::NotUnicode(_, _) => panic!("Incorrect format of TELEGRAM_BOT_TOKEN env variable")
                }
            }
        }

        let logs = Layer::default().with_filter(LevelFilter::from_level(self.debug_level));

        let format = tracing_subscriber::fmt::format()
            .without_time()
            .with_ansi(false)
            .compact();

        let bot_level = self.bot_level.clone();
        let bot_informer = Layer::default()
            .event_format(format)
            .with_writer(self)
            .with_filter(LevelFilter::from_level(bot_level));

        if let Err(err) = tracing::subscriber::set_global_default(
            Registry::default().with(logs).with(bot_informer),
        ) {
            panic!(
                "Ошибка при подключении глобального подписчика консольного вывода: {:#?}",
                err
            );
        };
    }
}

impl<'a> MakeWriter<'a> for TracingTgBotSubscriber {
    type Writer = BotWriter;

    fn make_writer(&'a self) -> Self::Writer {
        let token = match &self.token {
            Some(token) => token,
            None => panic!(
                "Please set up TELEGRAM_BOT_TOKEN env variable or use .set_token method for that"
            ),
        };
        BotWriter::new(token, self.user_id.unwrap())
    }

    fn make_writer_for(&'a self, _meta: &Metadata<'_>) -> Self::Writer {
        self.make_writer()
    }
}
