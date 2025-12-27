use crate::data::env_names::EnvNames;
use log::error;
use std::env;
use std::process::exit;

#[derive(Debug)]
pub struct EnvValidator {
    required_env_vars: Vec<EnvNames>,
}

impl EnvValidator {
    pub fn new(vars: Vec<EnvNames>) -> Self {
        EnvValidator {
            required_env_vars: vars,
        }
    }
    pub fn default() -> Self {
        let vars = EnvNames::all();
        EnvValidator {
            required_env_vars: vars,
        }
    }

    pub fn validate(&self) {
        let mut requires_exit = false;
        for env_var in &self.required_env_vars {
            let var = env_var.as_str();
            if env::var(var).is_err() {
                error!("❌ Missing required environment variable: {}", var);
                requires_exit = true;
            }
        }
        if requires_exit {
            exit(1);
        }
    }
}
