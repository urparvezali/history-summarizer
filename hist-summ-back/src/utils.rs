use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

const SECRET: &[u8; 15] = b"my_super_secret";

pub async fn encode_jwt(id: i64, minute: u64) -> String {
    let exp = (Utc::now() + Duration::minutes(minute as i64)).timestamp() as u64;
    let claim = Claim { exp, id };
    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(SECRET),
    )
    .unwrap()
}

pub async fn decode_jwt(
    token: &str,
) -> Result<jsonwebtoken::TokenData<Claim>, jsonwebtoken::errors::Error> {
    decode::<Claim>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claim {
    pub exp: u64,
    pub id: i64,
}
