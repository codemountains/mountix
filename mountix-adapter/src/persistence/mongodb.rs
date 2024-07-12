use std::env;
use std::sync::Arc;

use mongodb::{Client, Database};

#[derive(Clone)]
pub struct Db(pub(crate) Arc<Database>);

impl Db {
    pub async fn new() -> Db {
        let uri = env::var("DATABASE_URL").expect("DATABASE_URL is undefined.");
        let db_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "mountix_db".to_string());

        let client = Client::with_uri_str(&uri)
            .await
            .expect("Could not connect to MongoDB.");
        let db = client.database(&db_name);
        Db(Arc::new(db))
    }
}
