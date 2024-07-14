use crate::bot_writer::BotWriter;
use telegram_bot::Api;
use tracing::Metadata;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;
use tracing_subscriber::{
    filter::LevelFilter,
    fmt::{Layer, MakeWriter},
};

mod bot_writer;
mod config;

static TELEGRAM_USER_ID: &str = "TELEGRAM_USER_ID";

pub struct TracingTgBotSubscriber {
    api: Api,
    user_id: Option<i64>,
    /// Level of bot tracing level
    bot_level: tracing::Level,
    /// Level of debug process tracing level
    debug_level: tracing::Level,
    /// Disable bot if we don't wish to use it
    bot_enabled: bool,
}

pub fn new(api: Api) -> TracingTgBotSubscriber {
    TracingTgBotSubscriber::new(api)
}

impl TracingTgBotSubscriber {
    fn new(api: Api) -> Self {
        TracingTgBotSubscriber {
            api,
            user_id: None,
            bot_level: tracing::Level::ERROR,
            debug_level: tracing::Level::WARN,
            bot_enabled: true,
        }
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

    pub fn disable_bot(mut self) -> Self {
        self.bot_enabled = false;
        self
    }

    pub fn register(mut self) {
        if let None = self.user_id {
            match config::get_var(TELEGRAM_USER_ID) {
                Ok(user_id) => match user_id.parse::<i64>() {
                    Ok(user_id) => self.user_id = Some(user_id),
                    Err(_) => panic!(
                        "Incorrect format for {TELEGRAM_USER_ID} variable. Must be i64 number"
                    ),
                },
                Err(err) => match err {
                    config::ConfigError::NotPresent(_) => {}
                    config::ConfigError::NotUnicode(_, _) => {
                        panic!("Incorrect format of {TELEGRAM_USER_ID} env variable")
                    }
                },
            }
        }

        let logs = Layer::default().with_filter(LevelFilter::from_level(self.debug_level));

        let format = tracing_subscriber::fmt::format()
            .without_time()
            .with_ansi(false)
            .compact();

        match self.user_id {
            Some(_) => {
                let bot_level = self.bot_level.clone();

                if self.bot_enabled {
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
                } else {
                    if let Err(err) =
                        tracing::subscriber::set_global_default(Registry::default().with(logs))
                    {
                        panic!(
                            "Ошибка при подключении глобального подписчика консольного вывода: {:#?}",
                            err
                        );
                    };
                }
            }
            None => {
                if let Err(err) =
                    tracing::subscriber::set_global_default(Registry::default().with(logs))
                {
                    panic!(
                        "Ошибка при подключении глобального подписчика консольного вывода: {:#?}",
                        err
                    );
                };
            }
        }
    }
}

impl<'a> MakeWriter<'a> for TracingTgBotSubscriber {
    type Writer = BotWriter;

    fn make_writer(&'a self) -> Self::Writer {
        BotWriter::new(self.api.clone(), self.user_id.unwrap())
    }

    fn make_writer_for(&'a self, _meta: &Metadata<'_>) -> Self::Writer {
        self.make_writer()
    }
}
