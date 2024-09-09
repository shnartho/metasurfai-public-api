use crate::domain::model::ads::Ads;
use crate::infrastructure::repository::ads_repository::AdsRepository;

pub struct AdsService {
    repo: AdsRepository,
}

impl AdsService {
    pub fn new() -> Self {
        AdsService {
            repo: AdsRepository::new(),
        }
    }

    pub async fn test_get_ads(&self) -> Result<Vec<Ads>, String> {
        Ok(self.repo.generate_ads().await)
    }

    pub async fn get_ads(&self) -> Result<Vec<Ads>, Box<dyn std::error::Error>> {
        Ok(self.repo.fetch_ads_from_db().await?)
    }
}
