use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    username: String,
    email: String,
    balance: f64,
    region: String,
    country: String,
    safebrowse: bool
}

impl Profile {
    pub fn new(username: String, email: String, balance: f64, region: String, country: String, safebrowse: bool) -> Self {
        Profile {
            username,
            email,
            balance,
            region,
            country,
            safebrowse,
        }
    }
}