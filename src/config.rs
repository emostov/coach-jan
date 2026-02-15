use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub anthropic_api_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:data/coachjan.db?mode=rwc".to_string()),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            anthropic_api_key: env::var("ANTHROPIC_API_KEY").ok(),
        }
    }

    pub fn listen_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        // Clear env vars that might interfere
        unsafe {
            env::remove_var("DATABASE_URL");
            env::remove_var("HOST");
            env::remove_var("PORT");
            env::remove_var("ANTHROPIC_API_KEY");
        }

        let config = Config::from_env();
        assert_eq!(config.database_url, "sqlite:data/coachjan.db?mode=rwc");
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
        assert!(config.anthropic_api_key.is_none());
    }

    #[test]
    fn test_listen_addr() {
        let config = Config {
            database_url: String::new(),
            host: "127.0.0.1".to_string(),
            port: 8080,
            anthropic_api_key: None,
        };
        assert_eq!(config.listen_addr(), "127.0.0.1:8080");
    }
}
