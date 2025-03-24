use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};

pub async fn authenticate(request: Request, next: Next) -> Result<Response, StatusCode> {
    let _token = match request.headers().get(header::AUTHORIZATION) {
        Some(val) => match val.to_str().unwrap().split_whitespace().nth(1) {
            Some(val) => val,
            None => return Err(StatusCode::UNAUTHORIZED),
        },
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // authentication of the token should be here..

    Ok(next.run(request).await)
}
