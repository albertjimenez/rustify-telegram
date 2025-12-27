#[derive(Debug, Copy, Clone)]
pub enum EnvNames {
    TelegramChatId,
    GotifyClientToken,
    TelegramBotToken,
    Port,
}
impl EnvNames {
    pub const fn as_str(self) -> &'static str {
        match self {
            EnvNames::TelegramChatId => "TELEGRAM_CHAT_ID",
            EnvNames::GotifyClientToken => "GOTIFY_CLIENT_TOKEN",
            EnvNames::TelegramBotToken => "TELEGRAM_BOT_TOKEN",
            EnvNames::Port => "PORT",
        }
    }
}