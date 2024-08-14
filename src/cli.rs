use crate::errors::AppError;
use std::env;

pub fn get_url() -> Result<String, AppError> {
    env::args().nth(1).ok_or(AppError::MissingUrl)
}
