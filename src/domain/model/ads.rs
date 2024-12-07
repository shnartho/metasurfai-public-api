use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ads {
    pub _id: String,
    pub title: String,
    pub image_url: String,
    pub view_count: u64,
    pub description: String,
    pub posted_by: String,
    pub active: bool,
    pub max_views: i64,
    pub region: String,
    pub token_reward: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAdResponse {
    pub _id: String,
    pub title: String,
    pub image_url: String,
    pub view_count: u64,
    pub description: String,
    pub posted_by: String,
    pub active: bool,
    pub max_views: i64,
    pub region: String,
    pub token_reward: f64,
}

impl Ads {
    pub fn new(
        _id: String,
        title: String,
        image_url: String,
        view_count: u64,
        description: String,
        posted_by: String,
        active: bool,
        max_views: i64,
        region: String,
        token_reward: f64,
    ) -> Self {
        Ads {
            _id,
            title,
            image_url,
            view_count,
            description,
            posted_by,
            active,
            max_views,
            region,
            token_reward,
        }
    }
}
