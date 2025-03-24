#![allow(unused)]
use axum::{
    Router,
    routing::{get, post},
};

use crate::{unprotected::signup::signup, AppState};


pub fn unprotected_routers() -> Router<AppState>   {
    Router::new()
        .route("/users/signup", post(signup))
        // .route("/users/login", post(login))
}

pub fn protected_routers() -> Router<AppState>  {
    Router::new()
        // .route("/links/create_link", post(create_link))
        // .route("/links/retrive_link_all", get(retrieve_link_all))
}
