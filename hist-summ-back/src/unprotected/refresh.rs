use axum::{Json, extract::State, http::StatusCode};
use tower_cookies::Cookies;

use crate::{
    AppState,
    utils::{decode_jwt, encode_jwt},
};

use super::login::IdToken;

pub async fn refresh_rf(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<Json<IdToken>, StatusCode> {
    let refresh_token = cookies
        .get("refresh-token")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_string()
        .trim_start_matches("refresh-token=")
        .to_string();
    let claim = decode_jwt(&refresh_token, state.secret.as_bytes())
        .await
        .map_err(|_| {
            println!("refresh-token expired!");
            StatusCode::UNAUTHORIZED
        })?
        .claims;
    println!("refreshed token: {}", claim.id);
    Ok(Json(IdToken {
        id: claim.id,
        token: encode_jwt(claim.id, 2, state.secret.as_bytes()).await,
    }))
}
