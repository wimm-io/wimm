use thiserror::Error;

#[derive(Error, Debug)]
pub enum WimmError {
    #[error("Database error: {0}")]
    DbError(String),
}
