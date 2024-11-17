use crate::domain::model::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub image: Option<String>,
    pub email: String,
    pub ads: Option<Vec<String>>,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub verified: bool,
    pub location: Option<String>,
    pub balance: Option<f64>,
}

impl From<User> for Profile {
    fn from(user: User) -> Self {
        Profile {
            image: user.image,
            email: user.email,
            ads: user.ads,
            created_at: user.created_at,
            updated_at: user.updated_at,
            verified: user.verified,
            location: user.location,
            balance: user.balance,
        }
    }
}
