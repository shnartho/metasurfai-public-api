use crate::domain::model::user::User;

pub fn find_user_by_username(username: &str) -> Option<User> {
    if username == "jemal" {
        Some (User {
            username: "jemal".to_string(),
            password: "muntasir".to_string(),
        })
    } else {
        None
    }
}