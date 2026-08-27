use crate::{auth::service::AuthService, errors::AppError, models::AppState};
use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};

/// Axum Middleware to verify JWT Authorization header and attach Claims
pub async fn require_auth(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| {
            AppError::Unauthorized("Missing or invalid Authorization header".to_string())
        })?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized(
            "Authorization format must be 'Bearer <token>'".to_string(),
        ));
    }

    let token = &auth_header[7..];
    let claims = AuthService::verify_token(token, &state.config.jwt_secret)?;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
