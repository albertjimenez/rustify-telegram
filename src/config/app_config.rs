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
