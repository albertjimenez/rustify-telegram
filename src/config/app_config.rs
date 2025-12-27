use crate::data::env_names::EnvNames;

#[derive(Clone)]
pub struct AppConfig {
    pub gotify_token: String,
    pub telegram_token: String,
    pub telegram_chat_id: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            gotify_token: EnvNames::GotifyClientToken.get(),
            telegram_token: EnvNames::TelegramBotToken.get(),
            telegram_chat_id: EnvNames::TelegramChatId.get(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn loads_config_from_env() {
        unsafe { env::set_var("TELEGRAM_CHAT_ID", "123"); }
        unsafe { env::set_var("GOTIFY_CLIENT_TOKEN", "gotify"); }
        unsafe { env::set_var("TELEGRAM_BOT_TOKEN", "telegram"); }

        let cfg = AppConfig::from_env();

        assert_eq!(cfg.telegram_chat_id, "123");
        assert_eq!(cfg.gotify_token, "gotify");
        assert_eq!(cfg.telegram_token, "telegram");
    }
}
