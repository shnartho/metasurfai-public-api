use crate::{domain::service::billboard_service::Billboard, infrastructure::repository::mongodb_repo::MongodbRepository};

pub struct BillboardRepository {
    db: MongodbRepository,
}

impl BillboardRepository {
    pub fn new(db_client: MongodbRepository) -> Self {
        BillboardRepository { db: db_client }
    }

    pub async fn fetch_billboards_from_db(&self) -> Result<Vec<Billboard>, Box<dyn std::error::Error>> {
        self.db.get_billboards_from_db().await
    }
}