use crate::application::router::middlewares::jwt::create_token;
use crate::infrastructure::repository::user_repository::find_user_by_username;

pub fn authenticate(username: &str, password: &str) -> Result<String, &'static str> {
    if let Some(user) = find_user_by_username(username) {
        if user.password == password {
            let token = create_token(username);
            Ok(token)
        } else {
            Err("Invalid password")
        }
    } else {
        Err("User not found")
    }
}
