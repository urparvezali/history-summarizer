use axum::http::StatusCode;
use tower_cookies::Cookies;

use crate::utils::{decode_jwt, encode_jwt};

pub async fn refresh_rf(cookies: Cookies) -> Result<String, StatusCode> {
    let refresh_token = cookies
        .get("refresh-token")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_string()
        .trim_start_matches("refresh-token=")
        .to_string();
    let claim = decode_jwt(&refresh_token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .claims;
    Ok(encode_jwt(claim.id, 2).await)
}
