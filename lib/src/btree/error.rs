#[derive(thiserror::Error, Debug)]
pub enum BTreeError {
    #[error("Generic {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
