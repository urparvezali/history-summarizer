use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use crate::{AppState, utils::decode_jwt};

pub async fn authenticate(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // println!("{:#?}", request.headers().values());
    let claim = decode_jwt(token.token(), state.secret.as_bytes())
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(claim.claims);
    Ok(next.run(request).await)
}
