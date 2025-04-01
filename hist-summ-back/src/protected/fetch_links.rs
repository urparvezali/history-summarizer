use axum::{
    Extension, Json,
    extract::{Query, State},
    http::StatusCode,
};
use chrono::NaiveDate;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::{AppState, entities::links, utils::Claim};

pub async fn fetch_links(
    State(state): State<AppState>,
    Query(query): Query<FetchLinkQuery>,
    Extension(claim): Extension<Claim>,
) -> Result<Json<Vec<links::Model>>, StatusCode> {
    let mut condition = Condition::any();
    for s in query.searchkey.split_whitespace() {
        condition = condition.add(links::Column::Keywords.like(format!("%{s}%")));
    }

    let mut sttmnt = links::Entity::find().filter(links::Column::UserId.eq(claim.id));
    if let Some(naive_date) = query.from {
        sttmnt = sttmnt
            .clone()
            .filter(links::Column::CreatedAt.gte(naive_date));
    }
	
    let res = sttmnt
        .filter(condition)
        .all(state.db.as_ref())
        .await
        .unwrap();
    Ok(Json(res))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FetchLinkQuery {
    pub searchkey: String,
    pub from: Option<NaiveDate>,
}
