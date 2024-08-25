use mongodb::{options::ClientOptions, Collection, bson::doc, error::Error, Client};
use crate::domain::model::user::User;

pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub async fn new(uri: &str, db_name: &str, collection_name: &str) -> Result<Self, Error> {
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;

        let db = client.database(db_name);
        let collection = db.collection::<User>(collection_name);
        Ok(UserRepository { collection })
    }

    pub async fn login_user(&self, email: String, _password: String) -> Result<Option<User>, Error> {
        let filter = doc! {
            "email": email
        };
        if let Some(user) = self.collection.find_one(filter, None).await? {
            return Ok(Some(user));
        }
        Ok(None) 
    }
}


#[cfg(test)]
mod tests {
    use super::UserRepository;
    use std::env;

    async fn setup_test_db() -> UserRepository {
        let username = env::var("SECRET_MONGO_USERNAME").unwrap_or_else(|_| "msaiapp".to_string());
        let password = env::var("SECRET_PASSWORD").unwrap_or_else(|_| "rq17lyKhZFG8G8GI".to_string());
        let uri = env::var("MONGO_URI").unwrap_or_else(|_| {
            format!("mongodb+srv://{}:{}@cluster0.fvbgj.mongodb.net/sample_mflix?authSource=admin", username, password)
        });
        let db_name = "sample_mflix";
        let collection_name = "users";
        
        print!("-> {}", uri);
        let repo = UserRepository::new(&uri, &db_name, &collection_name).await.expect("Failed to initialize repository");
        repo  
    }

    #[tokio::test]
    async fn login_success_200_mongo() {
        let repo = setup_test_db().await;
        let user = repo.login_user("sean_bean@gameofthron.es".to_string(), "$2b$12$UREFwsRUoyF0CRqGNK0LzO0HM/jLhgUCNNIJ9RJAqMUQ74crlJ1Vu".to_string()).await.expect("Login failed");
        let user = user.unwrap();
        println!("{:?}", user);
        assert_eq!(user.password, "$2b$12$UREFwsRUoyF0CRqGNK0LzO0HM/jLhgUCNNIJ9RJAqMUQ74crlJ1Vu");
    }
}


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
