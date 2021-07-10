use http::{HeaderMap, HeaderValue};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    #[error("Invalid header value: {}", source)]
    HeaderValue {
        #[from]
        source: http::header::InvalidHeaderValue,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct Auth {
    pub(crate) token: Option<String>,
}

impl Auth {
    pub fn set_auth_header<'a>(
        &self,
        headers: &'a mut HeaderMap<HeaderValue>,
    ) -> Result<&'a mut HeaderMap<HeaderValue>, AuthError> {
        if let Some(ref api_key) = self.token {
            let mut val = HeaderValue::from_str(api_key)?;
            val.set_sensitive(true);
            headers.insert("X-API-Key", val);
        }

        Ok(headers)
    }
}
