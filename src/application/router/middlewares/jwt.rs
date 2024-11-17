use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User email
    pub exp: usize,  // Expiration timestamp (UNIX epoch)
}

const SECRET: &[u8] = b"secret_key";

/// Create a JWT token with a 1-hour expiration
pub fn create_token(user_email: &str) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        + 60 * 60; // Token expires in 1 hour

    let claims = Claims {
        sub: user_email.to_string(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .expect("Token creation failed")
}

/// Validate the token and ensure it hasn't expired
pub fn validate_token(token: &str) -> Result<Claims, String> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )
    .map_err(|e| e.to_string())?;

    // Check if the token is expired
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;

    if token_data.claims.exp < now {
        return Err("Token has expired".into());
    }

    Ok(token_data.claims)
}
