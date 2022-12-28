#[derive(thiserror::Error, Debug)]
pub enum VMError {
    #[error("Generic {0}")]
    Generic(String),
}
