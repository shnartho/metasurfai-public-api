use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ads {
    title: String,
    image_url: String,
    view_count: u64,
    description: String,
    posted_by: String,
    active: bool,
    max_views: i64,
    region: String,
    token_reward: f64,
}

impl Ads {
    pub fn new(
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
