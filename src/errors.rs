use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Missing URL. Usage: xyz-dlp [URL]")]
    MissingUrl,
    #[error("Failed to fetch HTML: {0}")]
    FetchError(#[from] reqwest::Error),
}
