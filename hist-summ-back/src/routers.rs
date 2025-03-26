#![allow(unused)]
use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    AppState,
    unprotected::{login::login, refresh::refresh_rf, signup::signup},
};

pub fn unprotected_routers() -> Router<AppState> {
    Router::new()
        .route("/users/signup", post(signup))
        .route("/users/login", post(login))
        .route("/users/refresh", get(refresh_rf))
}

pub fn protected_routers() -> Router<AppState> {
    Router::new().route("/users/{id}", get(crate::protected::read_user::read_user))
}
