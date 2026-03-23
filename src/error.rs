use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContinenteError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Failed to parse HTML from {url}: {message}")]
    Parse { url: String, message: String },

    #[error("No results found")]
    NoResults,

    #[error("Invalid configuration: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, ContinenteError>;
