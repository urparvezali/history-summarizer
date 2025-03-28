use axum::{Router, middleware};
use middlewares::authenticate;
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

pub mod entities;
pub mod middlewares;
pub mod protected;
pub mod routers;
pub mod unprotected;
pub mod utils;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub secret: Arc<String>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let secret = std::env::var("SECRET").expect("SECRET must be set in .env");
    let state = AppState {
        db: Arc::new(
            Database::connect(database_url)
                .await
                .expect("Database cant be connected!!"),
        ),
        secret: Arc::new(secret),
    };
    let tcp_listener = TcpListener::bind("localhost:8000")
        .await
        .expect("PORT: 8000 not available!!");

    let app = Router::new()
        .merge(routers::protected_routers())
        .layer(middleware::from_fn_with_state(state.clone(), authenticate))
        .merge(routers::unprotected_routers())
        .layer(
            CorsLayer::new()
                .allow_credentials(true)
                .allow_methods(AllowMethods::mirror_request())
                .allow_headers(AllowHeaders::mirror_request())
                .allow_origin(AllowOrigin::mirror_request()),
            // CorsLayer::very_permissive(),
        )
        .layer(CookieManagerLayer::new())
        .with_state(state.clone());

    axum::serve(tcp_listener, app)
        .await
        .expect("Unexpected Err");
}
