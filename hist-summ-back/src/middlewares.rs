use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use crate::utils::decode_jwt;

pub async fn authenticate(
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    decode_jwt(token.token())
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .claims;

    Ok(next.run(request).await)
}
