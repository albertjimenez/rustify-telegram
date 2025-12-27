use log::error;
use std::env;
use std::process::exit;

#[derive(Debug)]
pub struct EnvValidator {
    required_env_vars: Vec<String>,
}

impl EnvValidator {
    pub fn new(vars: Vec<String>) -> Self {
        EnvValidator {
            required_env_vars: vars,
        }
    }
    pub fn default() -> Self {
        let vars: Vec<String> = vec![
            "TELEGRAM_CHAT_ID".to_string(),
            "GOTIFY_CLIENT_TOKEN".to_string(),
            "TELEGRAM_BOT_TOKEN".to_string(),
            "PORT".to_string(),
        ];
        EnvValidator {
            required_env_vars: vars,
        }
    }

    pub fn validate(&self) {
        let mut requires_exit = false;
        for var in &self.required_env_vars {
            if let Err(_) = env::var(var) {
                let err_msg = format!("❌ Missing required environment variable: {}", var);
                error!("{}", err_msg);
                requires_exit = true;
            }
        }
        if requires_exit {
            exit(1);
        }
    }
}
