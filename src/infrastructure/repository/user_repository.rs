use crate::domain::model::user::User;

pub fn find_user_by_username(username: &str) -> Option<User> {
    if username == "nayem" {
        Some (User {
            username: "nayem".to_string(),
            password: "123".to_string(),
        })
    } else {
        None
    }
}