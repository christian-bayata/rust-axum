pub use crate::model::ModelManager;
use crate::model::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow; // Read data from sqlx and translate to struct


// Region:    --- Task Types
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForCreate {
    pub title: String
}

#[derive(Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>
} 

// Region:    --- Task BMC
pub struct TaskBmc;

impl TaskBmc {
    pub async fn create(
        _ctx: &Ctx,
        mm: &ModelManager,
        task_c: TaskForCreate
    ) -> Result<i64> {
        let db = mm.db();

        let (id, ) = sqlx::query_as::<_, (i64,)>(
            "INSERT INTO task (title) values ($1) returning id"
        ).bind(task_c.title)
        .fetch_one(db)
        .await?;

        todo!()
    }
}

/* Unit Tests */
#[cfg(test)]
mod tests { 
    #![allow(unused)]
    use super::*;
    use anyhow::Result;

    #[tokio::test] 
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_util::
        Ok(())
    }
}