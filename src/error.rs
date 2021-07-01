use thiserror::Error;

use crate::api;

pub type SpeedrunApiResult<T> = Result<T, SpeedrunApiError>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SpeedrunApiError {
    #[error("api error: {}", source)]
    Api {
        #[from]
        source: api::ApiError<RestError>,
    },
    #[error("url parse error: {}", source)]
    Parse {
        #[from]
        source: url::ParseError,
    },
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RestError {
    #[error("communication: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },
    #[error("http error: {}", source)]
    Http {
        #[from]
        source: http::Error,
    },
}
