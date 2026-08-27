use crate::errors::AppError;
use crate::models::{Claims, UserRole};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

pub struct AuthService;

impl AuthService {
    /// Hashes plain-text password securely using Argon2id algorithm
    pub fn hash_password(password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| {
                tracing::error!("Password hashing failure: {}", e);
                AppError::InternalServerError("Failed to secure password".to_string())
            })
    }

    /// Verifies plain-text password against stored Argon2id password hash
    pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(password_hash).map_err(|e| {
            tracing::error!("Invalid password hash format: {}", e);
            AppError::InternalServerError("Invalid hash comparison format".to_string())
        })?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Encodes JWT claims into a signed token string
    pub fn generate_token(
        user_id: Uuid,
        email: &str,
        role: UserRole,
        jwt_secret: &str,
        expiration_hours: i64,
    ) -> Result<String, AppError> {
        let now = Utc::now().timestamp();
        let exp = now + (expiration_hours * 3600);

        let claims = Claims {
            sub: user_id,
            email: email.to_string(),
            role,
            exp,
            iat: now,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        )
        .map_err(|e| {
            tracing::error!("JWT token creation failure: {}", e);
            AppError::InternalServerError("Failed to generate authentication token".to_string())
        })
    }

    /// Decodes and validates a signed JWT token string
    pub fn verify_token(token: &str, jwt_secret: &str) -> Result<Claims, AppError> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &validation,
        )
        .map_err(|_| {
            AppError::Unauthorized("Invalid or expired authentication token".to_string())
        })?;

        Ok(token_data.claims)
    }
}
