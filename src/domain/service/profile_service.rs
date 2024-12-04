use crate::domain::error::AppError;
use crate::domain::model::profile::Profile;
use crate::infrastructure::repository::user_repository::UserRepository;

pub struct ProfileService {
    repo: UserRepository,
}

impl ProfileService {
    pub fn new(profile_repo: UserRepository) -> Self {
        ProfileService {
            repo: profile_repo,
        }
    }

    pub async fn get_profile_from_db(&self, email: String) -> Result<Option<Profile>, AppError> {
        match self.repo.find_user_by_email_from_db(email).await {
            Ok(Some(user)) => {
                let profile = Profile::from(user);
                Ok(Some(profile))
            }
            Ok(None) => Err(AppError::InvalidCredentials),
            Err(_) => Err(AppError::InvalidCredentials),
        }
    }
}
