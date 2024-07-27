use crate::infrastructure::repository::ads_repository;
use crate::domain::model::ads::Ads;

pub async fn get_ads_from_db() -> Result<Vec<Ads>, String> {
    Ok(ads_repository::get_ads_from_db())
}