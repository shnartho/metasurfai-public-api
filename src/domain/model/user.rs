use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub image: Option<String>,
    pub email: String,
    pub password: String,
    pub ads: Option<Vec<String>>,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub verified: bool,
    pub location: Option<String>,
    pub balance: Option<f64>,
}

impl User {
    pub fn new(email: String, password: String) -> User {
        User {
            image: None,
            email,
            password,
            ads: None,
            created_at: Utc::now().to_rfc3339(),
            updated_at: None,
            verified: false,
            location: None,
            balance: Some(0.0),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserResponse {
    pub email: String,
    message: String,
}
