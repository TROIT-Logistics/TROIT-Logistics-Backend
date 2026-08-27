use crate::errors::AppError;

pub fn validate_email(email: &str) -> Result<(), AppError> {
    let email = email.trim();
    if email.is_empty() {
        return Err(AppError::ValidationError(
            "Email address cannot be empty".to_string(),
        ));
    }
    if !email.contains('@') || !email.contains('.') {
        return Err(AppError::ValidationError(
            "Invalid email address format".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), AppError> {
    if password.len() < 8 {
        return Err(AppError::ValidationError(
            "Password must be at least 8 characters long".to_string(),
        ));
    }
    Ok(())
}
