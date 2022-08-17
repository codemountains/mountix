use crate::persistence::mongodb::Db;
use std::sync::Arc;

pub struct HealthCheckRepository {
    db: Arc<Db>,
}

impl HealthCheckRepository {
    pub fn new(db: Db) -> Self {
        Self { db: Arc::new(db) }
    }

    pub async fn check_mongo_db(&self) -> anyhow::Result<()> {
        let db = self.db.0.clone();
        let _ = db.list_collections(None, None).await?;
        Ok(())
    }
}
