use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};

use crate::{AppState, entities::users, utils::encode_jwt};

pub async fn login(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<IdToken>, StatusCode> {
    let res = users::Entity::find()
        .filter(
            Condition::all()
                .add(users::Column::Email.eq(payload.email))
                .add(users::Column::Password.eq(payload.password)),
        )
        .one(state.db.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let idtoken = IdToken {
        id: res.id,
        token: encode_jwt(res.id, 1, state.secret.as_bytes()).await,
    };
    let refresh_token = encode_jwt(res.id, 30, state.secret.as_bytes()).await;

    cookies.add(
        Cookie::build(("refresh-token", refresh_token))
            .path("/")
            .http_only(true)
            .build(),
    );
    println!("logged in");
    Ok(Json(idtoken))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct IdToken {
    pub id: i64,
    pub token: String,
}
