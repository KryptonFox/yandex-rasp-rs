use crate::errors::{ApiRequestError, YaRaspError};
use reqwest::{Response, StatusCode};
use serde::Deserialize;

pub async fn handle_response<T>(response: Response) -> Result<T, YaRaspError>
where
    T: for<'a> Deserialize<'a>,
{
    match response.status() {
        StatusCode::OK => Ok(response.json::<T>().await?),
        StatusCode::BAD_REQUEST => Err(YaRaspError::ApiBadRequest(
            response.json::<ApiRequestError>().await?,
        )),
        StatusCode::NOT_FOUND => Err(YaRaspError::ApiNotFound(
            response.json::<ApiRequestError>().await?,
        )),
        _ => Err(YaRaspError::ApiErrorCode(response.status().as_u16())),
    }
}
