#![allow(unused)]
use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    AppState,
    protected::{fetch_links::fetch_links, fetch_recent::fetch_recent},
    unprotected::{login::login, logout::logout, refresh::refresh_rt, signup::signup},
};

pub fn unprotected_routers() -> Router<AppState> {
    Router::new()
        .route("/users/signup", post(signup))
        .route("/users/login", post(login))
        .route("/users/refresh", get(refresh_rt))
        .route("/users/logout", post(logout))
}

pub fn protected_routers() -> Router<AppState> {
    Router::new()
        .route("/users/{id}", get(crate::protected::read_user::read_user))
        .route("/links/fetch/{count}", get(fetch_recent))
        .route("/links/fetch", get(fetch_links))
}
