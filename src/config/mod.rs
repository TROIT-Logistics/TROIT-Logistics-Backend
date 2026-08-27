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

        let app_port = env::var("APP_PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse::<u16>()
            .map_err(|e| format!("Invalid APP_PORT configuration: {}", e))?;

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
