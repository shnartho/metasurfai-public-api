use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;
use serde_json::json;
use std::env;

use crate::domain::model::ads::Ads;

pub struct MongodbRepository {
    access_token: String,
}

impl MongodbRepository {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let username = env::var("MONGODB_USERNAME").ok();
        let password = env::var("MONGODB_PASSWORD").ok();

        let (username, password) = match (username, password) {
            (Some(u), Some(p)) => (u, p),
            _ => {
                return Err("Missing env variables".into())
            }
        };

        let auth_url = "https://eu-west-2.aws.services.cloud.mongodb.com/api/client/v2.0/app/data-nzlzdhy/auth/providers/local-userpass/login";
        let auth_data = json!({
            "username": username,
            "password": password
        });

        let client = reqwest::Client::new();
        let auth_response = client
            .post(auth_url)
            .header(CONTENT_TYPE, "application/json")
            .json(&auth_data)
            .send()
            .await?;

        if auth_response.status() == StatusCode::OK {
            let auth_response_json: serde_json::Value = auth_response.json().await?;
            let access_token = auth_response_json["access_token"]
                .as_str()
                .ok_or("Failed to get access token")?
                .to_string();
            println!("Authentication successful. Access token obtained.");
            Ok(MongodbRepository { access_token })
        } else {
            eprintln!(
                "Authentication failed with status code: {}",
                auth_response.status()
            );
            eprintln!("{}", auth_response.text().await?);
            Err("Authentication failed".into())
        }
    }

    pub async fn get_ads_from_mongo_db(&self) -> Result<Vec<Ads>, Box<dyn std::error::Error>> {
        let api_url = "https://eu-west-2.aws.data.mongodb-api.com/app/data-nzlzdhy/endpoint/data/v1/action/find";
        let headers = {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
            headers.insert(
                AUTHORIZATION,
                format!("Bearer {}", self.access_token).parse().unwrap(),
            );
            headers
        };

        let payload = json!({
            "collection": "ads",
            "database": "appmsai",
            "dataSource": "Cluster0",
            "filter": {},
            "limit": 20
        });

        let client = reqwest::Client::new();
        let api_response = client
            .post(api_url)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        let api_response_status = api_response.status();
        if api_response_status == StatusCode::OK {
            let api_response_json: serde_json::Value = api_response.json().await?;
            println!("API response status code: {}", api_response_status);
            let ads: Vec<Ads> = serde_json::from_value(api_response_json["documents"].clone())?;
            Ok(ads)
        } else {
            eprintln!(
                "API request failed with status code: {}",
                api_response_status
            );
            eprintln!("{}", api_response.text().await?);
            Err("API request failed".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::ads::Ads;

    #[tokio::test]
    async fn test_mongodb_repository() -> Result<(), Box<dyn std::error::Error>> {
        let repo = MongodbRepository::new().await?;
        let ads: Vec<Ads> = repo.get_ads_from_mongo_db().await?;
        println!("Retrieved ads: {:?}", ads);
        assert!(!ads.is_empty(), "Expected non-empty ads list");
        Ok(())
    }
}

