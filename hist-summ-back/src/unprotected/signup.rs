use crate::{AppState, entities::users};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use serde::{Deserialize, Serialize};

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignupPayload>,
) -> Result<Json<users::Model>, StatusCode> {
	println!("{:?}",payload.clone());
    let model = users::ActiveModel {
        email: Set(payload.email),
        password: Set(payload.password),
        name: Set(payload.name),
        ..Default::default()
    };
    let res = model
        .insert(state.db.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(res))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignupPayload {
    pub email: String,
    pub password: String,
    pub name: String,
}
