use crate::domain::model::profile::Profile;

pub fn get_profile_from_db(username: &str) -> Option<Profile> {
    if username == "nayem" {
        Some(Profile::new(
            "nayem".to_string(),
            "nayem@gmail.com".to_string(),
            0.5,
            "Iberia".to_string(),
            "Portugal".to_string(),
            false,
        ))
    } else {
        Some(Profile::new(
            "".to_string(),
            "".to_string(),
            0.0,
            "Unknown".to_string(),
            "Unknown".to_string(),
            false,
        ))
    }
}
