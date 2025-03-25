use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};

use crate::utils::decode_jwt;

pub async fn authenticate(request: Request, next: Next) -> Result<Response, StatusCode> {
    let token = match request.headers().get(header::AUTHORIZATION) {
        Some(val) => match val.to_str().unwrap().split_whitespace().nth(1) {
            Some(val) => val,
            None => return Err(StatusCode::UNAUTHORIZED),
        },
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let decoded = decode_jwt(&token.to_string()).await.map_err(|_| StatusCode::UNAUTHORIZED)?.claims;

    Ok(next.run(request).await)
}
