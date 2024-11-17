use crate::application::router::middlewares::jwt::create_token;
use crate::domain::error::AppError;
use crate::domain::model::user::User;
use crate::infrastructure::repository::user_repository::UserRepository;

pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new() -> Self {
        UserService {
            repo: UserRepository::new(),
        }
    }

    pub async fn login_user(&self, user: User) -> Result<String, AppError> {
        match self
            .repo
            .find_user_by_email_from_db(user.email.clone())
            .await
        {
            Ok(Some(found_user)) => {
                if found_user.password == user.password {
                    Ok(create_token(&user.email))
                } else {
                    Err(AppError::InvalidCredentials)
                }
            }
            Ok(None) => Err(AppError::InvalidCredentials),
            Err(e) => Err(AppError::DatabaseOperationFailed),
        }
    }

    pub async fn signup_user(&self, user: User) -> Result<User, AppError> {
        let existing_user = self
            .repo
            .find_user_by_email_from_db(user.email.clone())
            .await
            .map_err(|_| AppError::DatabaseOperationFailed)?;

        if existing_user.is_some() {
            return Err(AppError::UserAlreadyExists);
        }

        self.repo
            .create_user_in_db(user)
            .await
            .map_err(|_| AppError::UserCreationFailed)
    }
}
