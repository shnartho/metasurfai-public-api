use serde::{Deserialize, Serialize};

use crate::infrastructure::repository::billboard_repo::BillboardRepository;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Billboard {
    pub _id: String,
    pub title: String,
    pub image_url: String,
    pub description: String,
    pub posted_by: String,
    pub active: bool,
    pub region: String,
    pub bid_price: f64,
    pub created_at: i64,
    pub expires_at: Option<i64>,
}

pub struct BillboardService {
    repo: BillboardRepository
}

impl BillboardService {
    pub fn new(billboard_repo: BillboardRepository) -> Self {
        BillboardService {
            repo: billboard_repo,
        }
    }

    pub async fn get_billboards(&self) -> Result<Vec<Billboard>, Box<dyn std::error::Error>> {
        self.repo.fetch_billboards_from_db().await
    }
}