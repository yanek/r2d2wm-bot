use std::{env, fmt};

pub enum Environment {
    Development,
    Production,
}

impl Environment {
    #[must_use]
    pub fn get() -> Self {
        match env::var("COMPILE_ENVIRONMENT") {
            Ok(environment) => match environment.as_str() {
                "dev" => Environment::Development,
                _ => Environment::Production,
            },
            Err(_) => Environment::Production,
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Environment::Development => write!(f, "development"),
            Environment::Production => write!(f, "production"),
        }
    }
}
