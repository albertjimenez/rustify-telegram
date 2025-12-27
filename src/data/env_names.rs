use std::env;
use std::process::exit;
use log::error;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum EnvNames {
    TelegramChatId,
    GotifyClientToken,
    TelegramBotToken,
    Port,
}
impl EnvNames {
    pub fn get(self) -> String {
        env::var(self.as_str()).unwrap_or_else(|_| {
            error!("Missing env var {}", self.as_str());
            exit(1);
        })
    }
    pub fn get_u16(self) -> u16 {
        self.get()
            .parse()
            .expect("PORT must be a valid u16")
    }
    pub const fn as_str(self) -> &'static str {
        match self {
            EnvNames::TelegramChatId => "TELEGRAM_CHAT_ID",
            EnvNames::GotifyClientToken => "GOTIFY_CLIENT_TOKEN",
            EnvNames::TelegramBotToken => "TELEGRAM_BOT_TOKEN",
            EnvNames::Port => "PORT",
        }
    }
    pub fn all() -> Vec<EnvNames> {
        EnvNames::iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use std::env;
    use crate::data::env_validator::EnvValidator;

    fn clear_env() {
        for key in [
            "TELEGRAM_CHAT_ID",
            "GOTIFY_CLIENT_TOKEN",
            "TELEGRAM_BOT_TOKEN",
            "PORT",
        ] {
            unsafe { env::remove_var(key); }
        }
    }

    #[test]
    fn validate_succeeds_when_all_vars_present() {
        clear_env();

        unsafe { env::set_var("TELEGRAM_CHAT_ID", "123"); }
        unsafe { env::set_var("GOTIFY_CLIENT_TOKEN", "token"); }
        unsafe { env::set_var("TELEGRAM_BOT_TOKEN", "bot"); }
        unsafe { env::set_var("PORT", "8396"); }

        let validator = EnvValidator::default();
        validator.validate(); // should not exit
    }


}
