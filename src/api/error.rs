use std::{any, error::Error};

use thiserror::Error;

/// Errors that occur when creating form data.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BodyError {
    /// Error serializing body data from form paramaters
    #[error("URL encode error: {0}")]
    UrlEncoded(#[from] serde_urlencoded::ser::Error),
    #[error("JSON encode error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Errors that occur from API endpoints.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// Error creating body data
    #[error("failed to create form data: {0}")]
    Body(#[from] BodyError),
    /// The client encountered an error.
    #[error("client error: {0}")]
    Client(E),
    /// JSON deserialization failed
    #[error("failed to parse JSON: {0}")]
    Json(#[from] serde_json::Error),
    /// The URL failed to parse.
    #[error("url parse error: {0}")]
    Parse(#[from] url::ParseError),
    /// Speedrun.com returned an error
    #[error("Speedrun.com server error: {0}")]
    SpeedrunApi(String),
    /// Speedrun.com returned an unknown error
    #[error("Unknown speedrun.com server error: {0:?}")]
    Unknown(serde_json::Value),
    /// Failed parsing data type from JSON
    #[error("Parsing type {} from JSON: {}", typename, source)]
    DataType {
        /// The source of the error
        source: serde_json::Error,
        /// The name of the type that could not be deserialized.
        typename: &'static str,
    },
    /// The endpoint requires an API key to use, but none was provided.
    #[error("Endpoint requires authentication, but no API key was provided")]
    RequiresAuthentication,
}

impl<E> ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// Create an API error from a client error
    pub fn client(source: E) -> Self {
        Self::Client(source)
    }

    pub(crate) fn from_speedrun_api(val: serde_json::Value) -> Self {
        // let val = val.pointer("/message");
        if let Some(val) = val.pointer("/message") {
            if let Some(msg) = val.as_str() {
                Self::SpeedrunApi(msg.into())
            } else {
                Self::Unknown(val.clone())
            }
        } else {
            Self::Unknown(val.clone())
        }
    }

    pub(crate) fn data_type<T>(source: serde_json::Error) -> Self {
        Self::DataType {
            source,
            typename: any::type_name::<T>(),
        }
    }
}
