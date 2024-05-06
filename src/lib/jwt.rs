use actix_web::{cookie::Cookie, guard::GuardContext};
use chrono::{Duration, TimeDelta, Utc};
use dotenvy::var;
use jsonwebtoken::{self, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub const DURATION: TimeDelta = Duration::days(365);
pub const COOKIE_NAME: &str = "token";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub username: String,
}

pub fn encode(username: String) -> String {
    let duration = Utc::now() + DURATION;

    let claims = Claims {
        username,
        exp: duration.timestamp() as usize,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(var("JWT_SECRET").unwrap().as_ref()),
    )
    .unwrap()
}

pub fn decode(token: &str) -> Option<Claims> {
    match jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(var("JWT_SECRET").unwrap().as_ref()),
        &Validation::default(),
    ) {
        Ok(token) => Some(token.claims),
        _ => None,
    }
}

pub fn guard(ctx: &GuardContext) -> bool {
    let cookie_header = ctx.head().headers.get("Cookies");

    match cookie_header {
        Some(cookies) => {
            let parsed = Cookie::parse(cookies.to_str().unwrap()).unwrap();
            match decode(parsed.value()) {
                Some(_) => true,
                _ => false,
            }
        }
        _ => false,
    }
}

pub fn get_expiration() -> i64 {
    let duration = Utc::now() + DURATION;

    duration.timestamp()
}
