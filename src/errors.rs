use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum YaRaspError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("API request 400 Bad Request")]
    ApiBadRequest(ApiRequestError),

    #[error("API request 404 Not Found")]
    ApiNotFound(ApiRequestError),

    #[error("Api request failed with code {0}")]
    ApiErrorCode(u16),
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ApiRequestError {
    pub error: Error,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub request: String,
    pub text: String,
    pub error_code: String,
    pub http_code: i64,
}
