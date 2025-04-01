use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

use crate::{AppState, entities::links, utils::Claim};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Url {
    pub id: i64,
    pub url: String,
}

pub async fn fetch_recent(
    State(state): State<AppState>,
    Extension(claim): Extension<Claim>,
    Path(count): Path<u64>,
) -> Result<Json<Vec<links::Model>>, StatusCode> {
    let res = links::Entity::find()
        .filter(links::Column::UserId.eq(claim.id))
        .order_by(links::Column::CreatedAt, sea_orm::Order::Desc)
        .limit(count)
        .all(state.db.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // println!("{:?}", res);
    Ok(Json(res))
}
