//! Error types for the crate
use thiserror::Error;

use crate::{api, AuthError};

/// An alias for result types returned by this crate.
pub type SpeedrunApiResult<T> = Result<T, SpeedrunApiError>;

//TODO: Make these variants instead of structs
/// Errors from the speedrun.com api client.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SpeedrunApiError {
    /// Error from the speedrun.com API
    #[error("API error: {source}")]
    Api {
        #[from]
        source: api::ApiError<RestError>,
    },
    /// Error parsing URL
    #[error("url parse error: {source}")]
    Parse {
        #[from]
        source: url::ParseError,
    },
}

//TODO: Make these variants instead of structs
/// Error communicating with the REST endpoint.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RestError {
    /// Reqwest client error
    #[error("communication: {source}")]
    Communication {
        #[from]
        source: reqwest::Error,
    },
    /// HTTP protocol error
    #[error("HTTP error: {source}")]
    Http {
        #[from]
        source: http::Error,
    },
    /// Authentication error
    #[error("Authentication error: {source}")]
    Authentication {
        #[from]
        source: AuthError,
    },
}
