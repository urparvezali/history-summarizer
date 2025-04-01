use std::{
    collections::HashSet,
    time::{SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

pub async fn encode_jwt(id: i64, minute: u64, secret: &[u8]) -> String {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 60 * minute;
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

pub fn counter(s1: &String, s2: &String) -> usize {
    let st1: HashSet<&str> = s1.split_whitespace().collect();
    let st2: HashSet<&str> = s2.split_whitespace().collect();
    st1.intersection(&st2).count()
}
