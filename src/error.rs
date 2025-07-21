use thiserror::Error;

use crate::genius;

#[derive(Debug, Error)]
pub enum Error {
    #[error("genius: {0}")]
    Genius(#[from] genius::Error),
    #[error("sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
}
