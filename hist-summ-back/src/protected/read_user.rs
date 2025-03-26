use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::EntityTrait;

use crate::{AppState, entities::users};

pub async fn read_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<users::Model>, StatusCode> {
    if let Some(user) = users::Entity::find_by_id(id)
        .one(state.db.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Ok(Json(user));
    }
    Err(StatusCode::NOT_FOUND)
}
