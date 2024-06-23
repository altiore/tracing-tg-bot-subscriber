use std::io::{Stdout, Write};
use std::sync::Arc;
use telegram_bot::*;
use tokio::task;

async fn send_to_admin(api: Arc<Api>, user_id: i64, text: String) {
    if let Err(err) = api.send(UserId::new(user_id).text(text)).await {
        println!("Не удалось отправить личное сообщение: {:#?}", err);
    }
}

pub struct BotWriter {
    admin_id: i64,
    api: Arc<Api>,
    writer: Stdout,
}
impl BotWriter {
    pub fn new(bot_key: &str, admin_id: i64) -> Self {
        BotWriter {
            admin_id,
            api: Arc::new(Api::new(bot_key)),
            writer: std::io::stdout(),
        }
    }
}
impl Write for BotWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        match std::str::from_utf8(buf) {
            Ok(text) => {
                let api = Arc::clone(&self.api);
                task::spawn(send_to_admin(api, self.admin_id, text.to_owned()));

                self.writer.write_all(buf)
            }
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err)),
        }
    }
}
