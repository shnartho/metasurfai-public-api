use crate::domain::model::profile::Profile;
use crate::infrastructure::repository::profile_repository;

pub async fn get_profile(username: &str) -> Option<Profile> {
    profile_repository::get_profile_from_db(username)
}
