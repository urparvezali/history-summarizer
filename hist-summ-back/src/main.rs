use axum::{Router, middleware};
use middlewares::authenticate;
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};

pub mod entity;
pub mod middlewares;
pub mod protected;
pub mod routers;
pub mod unprotected;
pub mod utils;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let shared_db = Arc::new(
        Database::connect(database_url)
            .await
            .expect("Database cant be connected!!"),
    );
    let tcp_listener = TcpListener::bind("localhost:8000")
        .await
        .expect("PORT: 8000 not available!!");

    let app = Router::new()
        .merge(routers::protected_routers())
        .layer(middleware::from_fn(authenticate))
        .merge(routers::unprotected_routers())
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_methods(Any)
                .allow_origin(Any)
                .allow_private_network(true),
        )
        .layer(CookieManagerLayer::new())
        .with_state(AppState { db: shared_db });

    axum::serve(tcp_listener, app)
        .await
        .expect("Unexpected Err");
}
