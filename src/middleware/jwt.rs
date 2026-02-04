use std::time::{SystemTime, UNIX_EPOCH};

use crate::apps::users::models::User;
use anyhow::Result;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user: User,
    pub exp: u64,
    pub iat: u64,
}

/// Generate a JWT token for a user
///
/// # Arguments
///
/// * `user` - The user to generate a token for
/// * `secret` - The secret to use for the token
/// * `expiry` - The expiry time for the token
///
/// # Returns
/// Returns the token if it is generated successfully
pub fn generate_token(
    user: &User,
    secret: &str,
    expiry: u64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let header = Header::default();

    // get the current time in seconds since the epoch
    let now = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_| {
        jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken)
    })?;

    let current_timestamp = now.as_secs();
    let expiration_time = current_timestamp + expiry;

    let claims = Claims {
        user: user.clone(),
        exp: expiration_time,
        iat: current_timestamp,
    };
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Verify a JWT token
///
/// # Arguments
///
/// * `token` - The token to verify
/// * `secret` - The secret to use for the token
///
/// # Returns
/// Returns the claims if the token is valid
pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
