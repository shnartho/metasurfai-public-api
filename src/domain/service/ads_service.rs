use crate::domain::model::ads::{Ads, CreateAdResponse};
use crate::infrastructure::repository::ads_repository::AdsRepository;

pub struct AdsService {
    repo: AdsRepository,
}

impl AdsService {
    pub fn new(ads_repo: AdsRepository) -> Self {
        AdsService {
            repo: ads_repo,
        }
    }

    pub async fn test_get_ads(&self) -> Result<Vec<Ads>, String> {
        Ok(self.repo.generate_ads().await)
    }

    pub async fn get_ads(&self) -> Result<Vec<Ads>, Box<dyn std::error::Error>> {
        self.repo.fetch_ads_from_db().await
    }

    pub async fn create_ads(
        &self,
        ad: Ads,
    ) -> Result<CreateAdResponse, Box<dyn std::error::Error>> {
        self.repo.create_ads_in_db(ad).await
    }

    pub async fn delete_ads(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        self.repo.delete_ads_in_db(id).await
    }
}
