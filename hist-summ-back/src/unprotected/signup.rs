use crate::{AppState, entities::users};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use serde::{Deserialize, Serialize};

pub async fn signup(
    State(state): State<AppState>,
    Json(user): Json<SignupPayload>,
) -> impl IntoResponse {
    let model = users::ActiveModel {
        email: Set(user.email),
        password: Set(user.password),
        name: Set(user.name),
        ..Default::default()
    };
    let res = model.insert(state.db.as_ref()).await.unwrap();
    (StatusCode::CREATED, Json(res))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupPayload {
    pub email: String,
    pub password: String,
    pub name: String,
}

