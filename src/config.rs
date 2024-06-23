use std::env;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConfigError {
    NotPresent(String),
    NotUnicode(String, String),
}

pub fn get_var(name: &str) -> Result<String, ConfigError> {
    match env::var(name) {
        Ok(token) => Ok(token),
        Err(err) => match err {
            env::VarError::NotPresent => Err(ConfigError::NotPresent(name.to_owned())),
            env::VarError::NotUnicode(ref s) => Err(ConfigError::NotUnicode(
                name.to_owned(),
                match s.to_str() {
                    Some(s) => s.to_owned(),
                    None => String::from("`[no symbols to show error]`"),
                },
            )),
        },
    }
}
