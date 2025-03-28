use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

pub async fn encode_jwt(id: i64, minute: u64, secret: &[u8]) -> String {
    let exp = (Utc::now() + Duration::minutes(minute as i64)).timestamp() as u64;
    let claim = Claim { exp, id };
    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret),
    )
    .unwrap()
}

pub async fn decode_jwt(
    token: &str,
    secret: &[u8],
) -> Result<jsonwebtoken::TokenData<Claim>, jsonwebtoken::errors::Error> {
    decode::<Claim>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Claim {
    pub exp: u64,
    pub id: i64,
}
