use crate::model::store;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    // -- Modules
    Store(store::Error),

    // -- Externals
    sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

// region:     ---  Froms 
impl From<sqlx::Error> for Error {
    fn from(val: sqlx::Error) -> Self {
        Self::sqlx(val)
    }
}

impl From<store::Error> for Error {
    fn from(val: store::Error) -> Self {
        Self::Store(val)
    }
}