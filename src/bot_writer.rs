use std::io::{Write};
use telegram_bot::*;
use tokio::task;

async fn send_to_admin(api: Api, user_id: i64, text: String) {
    if let Err(err) = api.send(UserId::new(user_id).text(text)).await {
        // TODO: записывать неудачные попытки отправки в файл
        println!("Не удалось отправить личное сообщение: {:#?}", err);
    }
}

pub struct BotWriter {
    admin_id: i64,
    api: Api,
}
impl BotWriter {
    pub fn new(api: Api, admin_id: i64) -> Self {
        BotWriter { admin_id, api }
    }
}
impl Write for BotWriter {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        match std::str::from_utf8(buf) {
            Ok(text) => {
                task::spawn(send_to_admin(
                    self.api.clone(),
                    self.admin_id,
                    text.to_owned(),
                ));

                Ok(())
            }
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err)),
        }
    }
}
