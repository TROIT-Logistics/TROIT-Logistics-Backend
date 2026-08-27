#[cfg(test)]
mod tests {
    use troit_logistics_backend::{
        auth::service::AuthService,
        models::UserRole,
        utils::{validate_email, validate_password},
    };
    use uuid::Uuid;

    #[test]
    fn test_email_validation() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("invalid-email").is_err());
        assert!(validate_email("").is_err());
    }

    #[test]
    fn test_password_validation() {
        assert!(validate_password("secure_pass123").is_ok());
        assert!(validate_password("short").is_err());
    }

    #[test]
    fn test_argon2_password_hashing_and_verification() {
        let plain_password = "super_secret_password_123";
        let hash = AuthService::hash_password(plain_password).expect("Hashing should succeed");

        assert_ne!(plain_password, hash);
        assert!(AuthService::verify_password(plain_password, &hash).unwrap_or(false));
        assert!(!AuthService::verify_password("wrong_password", &hash).unwrap_or(true));
    }

    #[test]
    fn test_jwt_token_generation_and_verification() {
        let user_id = Uuid::new_v4();
        let email = "seller@troitlogistics.com";
        let role = UserRole::Seller;
        let secret = "test_secret_key_123456789";
        let exp_hours = 24;

        let token = AuthService::generate_token(user_id, email, role, secret, exp_hours)
            .expect("Token generation should succeed");

        let claims =
            AuthService::verify_token(&token, secret).expect("Token verification should succeed");

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, email);
        assert_eq!(claims.role, UserRole::Seller);
    }
}
