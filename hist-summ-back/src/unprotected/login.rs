use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};

use crate::{AppState, entities::users, utils::encode_jwt};

pub async fn login(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<String, StatusCode> {
    let res = users::Entity::find()
        .filter(
            Condition::all()
                .add(users::Column::Email.eq(payload.email))
                .add(users::Column::Password.eq(payload.password)),
        )
        .one(state.db.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if res.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }
    let token = encode_jwt(res.clone().unwrap().id, 1).await;
    let refresh_token = encode_jwt(res.unwrap().id, 10).await;

    cookies.add(
        Cookie::build(("refresh-token", refresh_token))
            .path("/")
            .http_only(true)
            .build(),
    );

    Ok(token)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}
