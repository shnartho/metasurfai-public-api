use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;
use serde_json::json;

use crate::domain::model::ads::{Ads, CreateAdResponse};
use crate::domain::model::user::User;
use crate::domain::service::billboard_service::Billboard;

#[derive(Debug, Clone)]
pub struct MongodbRepository {
    access_token: String,
}

impl MongodbRepository {
    pub async fn new(username: String, password: String) -> Result<Self, Box<dyn std::error::Error>> {
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

    // +++++++++++++++++++++++++++++++++ Users ++++++++++++++++++++++++++++++++++
    // Users mongo repo
    pub async fn find_user_by_email_from_db(
        &self,
        email: String,
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let api_url = "https://eu-west-2.aws.data.mongodb-api.com/app/data-nzlzdhy/endpoint/data/v1/action/findOne";
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
            "collection": "users",
            "database": "appmsai",
            "dataSource": "Cluster0",
            "filter": {
                "email": email
            }
        });
    
        let client = reqwest::Client::new();
        let api_response = client
            .post(api_url)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;
    
        if api_response.status() != StatusCode::OK {
            return Err("API request failed".into());
        }
    
        let api_response_json: serde_json::Value = api_response.json().await?;
        dbg!(&api_response_json);
    
        match api_response_json.get("document") {
            Some(doc) if !doc.is_null() => {
                let user = serde_json::from_value::<User>(doc.clone())?;
                Ok(Some(user))
            }
            _ => Ok(None),
        }
    }
    

    pub async fn create_user_in_db(&self, user: User) -> Result<User, Box<dyn std::error::Error>> {
        let api_url =
            "https://data.mongodb-api.com/app/data-nzlzdhy/endpoint/data/v1/action/insertOne";
        let headers = {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(CONTENT_TYPE, "application/ejson".parse().unwrap());
            headers.insert(
                AUTHORIZATION,
                format!("Bearer {}", self.access_token).parse().unwrap(),
            );
            headers
        };

        let payload = json!({
            "dataSource": "Cluster0",
            "database": "appmsai",
            "collection": "users",
            "document": {
                "image": user.image,
                "email": user.email,
                "password": user.password,
                "ads": user.ads,
                "created_at": user.created_at,
                "updated_at": user.updated_at,
                "verified": user.verified,
                "location": user.location,
                "balance": user.balance
            }
        });

        let client = reqwest::Client::new();
        let api_response = client
            .post(api_url)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        match api_response.status() {
            StatusCode::CREATED => api_response
                .json::<serde_json::Value>()
                .await?
                .get("insertedId")
                .map(|_| User::new(user.email, user.password))
                .ok_or_else(|| "No 'insertedId' field in API response".into()),
            _ => Err("API request failed".into()),
        }
    }

    // +++++++++++++++++++++++++++++++++ Ads ++++++++++++++++++++++++++++++++++
    // Ads mongo repo
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
            let ads: Vec<Ads> = serde_json::from_value(api_response_json["documents"].clone())?;
            Ok(ads)
        } else {
            Err("API request failed".into())
        }
    }

    pub async fn create_ads_in_db(
        &self,
        ad: Ads,
    ) -> Result<CreateAdResponse, Box<dyn std::error::Error>> {
        let api_url =
            "https://data.mongodb-api.com/app/data-nzlzdhy/endpoint/data/v1/action/insertOne";
        let headers = {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(CONTENT_TYPE, "application/ejson".parse().unwrap());
            headers.insert(
                AUTHORIZATION,
                format!("Bearer {}", self.access_token).parse().unwrap(),
            );
            headers
        };

        let payload = json!({
            "dataSource": "Cluster0",
            "database": "appmsai",
            "collection": "ads",
            "document": {
                "title": ad.title,
                "image_url": ad.image_url,
                "view_count": ad.view_count,
                "description": ad.description,
                "posted_by": ad.posted_by,
                "active": ad.active,
                "max_views": ad.max_views,
                "region": ad.region,
                "token_reward": ad.token_reward
            }
        });

        let client = reqwest::Client::new();
        let api_response = client
            .post(api_url)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        let api_response_status = api_response.status();
        if api_response_status == StatusCode::CREATED {
            let api_response_json: serde_json::Value = api_response.json().await?;

            if let Some(inserted_id) = api_response_json.get("insertedId") {
                let inserted_ad = CreateAdResponse {
                    _id: ad._id,
                    title: ad.title,
                    image_url: ad.image_url,
                    view_count: ad.view_count,
                    description: ad.description,
                    posted_by: ad.posted_by,
                    active: ad.active,
                    max_views: ad.max_views,
                    region: ad.region,
                    token_reward: ad.token_reward,
                };

                Ok(inserted_ad)
            } else {
                Err("No 'insertedId' field in API response".into())
            }
        } else {
            Err("API request failed".into())
        }
    }

    pub async fn delete_ads_by_id(
        &self,
        ad_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let api_url =
            "https://data.mongodb-api.com/app/data-nzlzdhy/endpoint/data/v1/action/deleteOne";
        let headers = {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(CONTENT_TYPE, "application/ejson".parse().unwrap());
            headers.insert(
                AUTHORIZATION,
                format!("Bearer {}", self.access_token).parse().unwrap(),
            );
            headers
        };

        let payload = json!({
            "dataSource": "Cluster0",
            "database": "appmsai",
            "collection": "ads",
            "filter": {
                "_id": { "$oid": ad_id }
            }
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

            if let Some(deleted_count) = api_response_json.get("deletedCount") {
                Ok(format!("Deleted {} ad(s)", deleted_count))
            } else {
                Err("No 'deletedCount' field in API response".into())
            }
        } else {
            Err("API request failed".into())
        }
    }

    // +++++++++++++++++++++++++++++++++ billboard ++++++++++++++++++++++++++++++++++
    // billboard mongo repo
    pub async fn get_billboards_from_db(&self) -> Result<Vec<Billboard>, Box<dyn std::error::Error>> {
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
            "collection": "billboards",
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
            //println!("API response JSON: {}", api_response_json);
            let billboards: Vec<Billboard> = serde_json::from_value(api_response_json["documents"].clone())?;
            Ok(billboards)
        } else {
            println!("API request failed with status code: {}", api_response_status);
            Err("API request failed".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::ads::Ads;
    use crate::application::config::AppConfig;

    #[tokio::test]
    async fn test_get_ads_from_mongodb_repository() -> Result<(), Box<dyn std::error::Error>> {
        let config = AppConfig::load()?;
        let repo = MongodbRepository::new(config.mongo_username, config.mongo_password).await?;
        let ads: Vec<Ads> = repo.get_ads_from_mongo_db().await?;
        println!("Retrieved ads: {:?}", ads);
        assert!(!ads.is_empty(), "Expected non-empty ads list");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_billboards_from_mongodb_repository() -> Result<(), Box<dyn std::error::Error>> {
        let config = AppConfig::load()?;
        let repo = MongodbRepository::new(config.mongo_username, config.mongo_password).await?;
        let billboards: Vec<Billboard> = repo.get_billboards_from_db().await?;
        println!("Retrieved ads: {:?}", billboards);
        assert!(!billboards.is_empty(), "Expected non-empty ads list");
        Ok(())
    }

    #[tokio::test]
    async fn test_create_ads_in_db() -> Result<(), Box<dyn std::error::Error>> {
        let config = AppConfig::load()?;
        let repo = MongodbRepository::new(config.mongo_username, config.mongo_password).await?;
        let ad = Ads {
            _id: "66fdd9f89505c5e6423d1348".to_string(),
            title: "Test ad".to_string(),
            image_url: "https://test.com/image.jpg".to_string(),
            view_count: 0,
            description: "Test description".to_string(),
            posted_by: "Test user".to_string(),
            active: true,
            max_views: 100,
            region: "Test region".to_string(),
            token_reward: 10.0,
        };
        let ads = repo.create_ads_in_db(ad).await?;
        println!("Created ad: {:?}", ads);
        assert!(ads.title == "Test ad", "Expected title to be 'Test ad'");
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_ad_by_id() -> Result<(), Box<dyn std::error::Error>> {
        let config = AppConfig::load()?;
        let repo = MongodbRepository::new(config.mongo_username, config.mongo_password).await?;
        let test_ads_id = "66fdd9f89505c5e6423d1348";
        let delete_response = repo.delete_ads_by_id(test_ads_id).await?;
        println!("Delete response: {}", delete_response);
        assert!(
            delete_response.contains("Deleted"),
            "Expected delete response to contain 'Deleted'"
        );
        Ok(())
    }
}
