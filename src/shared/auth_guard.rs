use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use jsonwebtoken::{DecodingKey, Validation};

use crate::shared::utils::Claims;

pub struct JWTAuth {
    pub user_id: i64,
    pub role: String,
}

pub struct AuthError(StatusCode, String);

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for JWTAuth
where
    S: Send + Sync,
{
    type Rejection = AuthError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // ambil header Authorization
        let auth_handler = parts.headers.get("Authorization").ok_or(AuthError(
            StatusCode::UNAUTHORIZED,
            "token tidak ada".to_string(),
        ))?;

        // Parse "Bearer <token>"
        let auth_str = auth_handler
            .to_str()
            .map_err(|_| AuthError(StatusCode::UNAUTHORIZED, "token tidak valid".to_string()))?;
        if !auth_str.starts_with("Bearer ") {
            return Err(AuthError(
                StatusCode::UNAUTHORIZED,
                "token tidak valid".to_string(),
            ));
        }
        let token = &auth_str[7..];
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "rahasia".to_string());
        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AuthError(StatusCode::UNAUTHORIZED, "token expired".to_string()))?;
        Ok(JWTAuth {
            user_id: token_data.claims.sub.parse().unwrap(),
            role: token_data.claims.role,
        })
    }
}
