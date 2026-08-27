use std::env;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AppConfig {
    pub database_url: String,
    pub app_host: String,
    pub app_port: u16,
    pub rust_log: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        // Attempt to load .env file if available
        let _ = dotenvy::dotenv();

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@localhost:5432/troit_logistics".to_string()
        });

        let app_host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let app_port = env::var("PORT")
            .or_else(|_| env::var("APP_PORT"))
            .unwrap_or_else(|_| "8000".to_string())
            .parse::<u16>()
            .map_err(|e| format!("Invalid port configuration: {}", e))?;

        let rust_log = env::var("RUST_LOG")
            .unwrap_or_else(|_| "info,troit_logistics_backend=debug".to_string());

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
            "troit_local_dev_jwt_secret_key_change_in_prod_1234567890".to_string()
        });

        let jwt_expiration_hours = env::var("JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse::<i64>()
            .map_err(|e| format!("Invalid JWT_EXPIRATION_HOURS configuration: {}", e))?;

        Ok(Self {
            database_url,
            app_host,
            app_port,
            rust_log,
            jwt_secret,
            jwt_expiration_hours,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_port_resolution_priority() {
        let orig_port = env::var("PORT").ok();
        let orig_app_port = env::var("APP_PORT").ok();

        // 1. PORT takes priority over APP_PORT and default
        env::set_var("PORT", "9000");
        env::set_var("APP_PORT", "7000");
        let config = AppConfig::from_env().expect("Config loading failed");
        assert_eq!(config.app_port, 9000);

        // 2. APP_PORT takes priority when PORT is not set
        env::remove_var("PORT");
        env::set_var("APP_PORT", "7000");
        let config = AppConfig::from_env().expect("Config loading failed");
        assert_eq!(config.app_port, 7000);

        // 3. Default 8000 is used when neither PORT nor APP_PORT is set
        env::remove_var("PORT");
        env::remove_var("APP_PORT");
        let config = AppConfig::from_env().expect("Config loading failed");
        assert_eq!(config.app_port, 8000);

        // Clean up environment variables
        if let Some(val) = orig_port {
            env::set_var("PORT", val);
        } else {
            env::remove_var("PORT");
        }
        if let Some(val) = orig_app_port {
            env::set_var("APP_PORT", val);
        } else {
            env::remove_var("APP_PORT");
        }
    }
}
