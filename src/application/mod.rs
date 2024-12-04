pub mod router;
pub mod config;

use std::sync::Arc;
use crate::domain::service::{ads_service::AdsService, profile_service::ProfileService, user_service::UserService};
use crate::infrastructure::repository::ads_repository::AdsRepository;
use crate::infrastructure::repository::mongodb_repo::MongodbRepository;
use crate::infrastructure::repository::user_repository::UserRepository;

#[derive(Clone)]
pub struct AppState {
    pub ads_service: Arc<AdsService>,
    pub profile_service: Arc<ProfileService>,
    pub user_service: Arc<UserService>
}

impl AppState {
    pub fn new(db_client: MongodbRepository) -> Self {
        AppState {
            ads_service: Arc::new(AdsService::new(AdsRepository::new(db_client.clone()))),
            profile_service: Arc::new(ProfileService::new(UserRepository::new(db_client.clone()))),
            user_service: Arc::new(UserService::new(UserRepository::new(db_client))),
        }
    }
}