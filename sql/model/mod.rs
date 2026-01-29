mod error;
mod store;

pub use self::error::{Error, Result};

#[derive(Clone)]
pub struct ModelManager {
    db: Db
}

impl ModelManager {
    // Constructor
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;
        
        Ok(ModelManager { db })
    }

    // Only code inside model module can access the DB pool
    // This enforces layer isolation at compile time.
    pub(in create::model) fn db(&self) -> &Db {
        &self.db
    }
}

