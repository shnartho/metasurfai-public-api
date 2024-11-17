use super::mongodb_repo::MongodbRepository;
use crate::domain::model::user::User;

pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        UserRepository {}
    }

    pub async fn find_user_by_email_from_db(
        &self,
        email: String,
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let db = MongodbRepository::new().await?;
        match db.find_user_by_email_from_db(email).await {
            Ok(user) => Ok(user),
            Err(e) => Err(e),
        }
    }

    pub async fn create_user_in_db(&self, user: User) -> Result<User, Box<dyn std::error::Error>> {
        let db = MongodbRepository::new().await?;
        match db.create_user_in_db(user).await {
            Ok(user) => Ok(user),
            Err(e) => Err(e),
        }
    }
}
