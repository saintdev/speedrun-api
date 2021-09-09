use std::convert::TryInto;

use async_trait::async_trait;
use futures::TryFutureExt;
use log::debug;
use reqwest::{blocking::Client as HttpClient, Client as AsyncHttpClient};
use url::Url;

use crate::{
    api,
    auth::Auth,
    error::{RestError, SpeedrunApiResult},
};

const SPEEDRUN_API_BASE_URL: &str = "https://www.speedrun.com/api/v1/";

/// A client for communicating with the Speedrun.com API
#[derive(Clone, Debug)]
pub struct SpeedrunApiClient {
    client: HttpClient,
    rest_url: Url,
    api_key: Auth,
}

impl SpeedrunApiClient {
    /// Create a new Speedrun.com API client.
    pub fn new() -> SpeedrunApiResult<Self> {
        Self::new_impl::<String>(None)
    }

    /// Create a new Speedrun.com API client, with the provided API key.
    pub fn with_api_key<S>(api_key: S) -> SpeedrunApiResult<Self>
    where
        S: Into<String>,
    {
        Self::new_impl(Some(api_key))
    }

    fn new_impl<S>(api_key: Option<S>) -> SpeedrunApiResult<Self>
    where
        S: Into<String>,
    {
        let rest_url = Url::parse(SPEEDRUN_API_BASE_URL)?;
        let api_key = Auth {
            token: api_key.map(Into::into),
        };

        Ok(SpeedrunApiClient {
            client: HttpClient::new(),
            rest_url,
            api_key,
        })
    }

    /// Create a new Speedrun.com API builder.
    pub fn builder() -> SpeedrunApiBuilder {
        SpeedrunApiBuilder::new()
    }
}

impl api::RestClient for SpeedrunApiClient {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        debug!("REST api call {}", endpoint);
        self.rest_url
            .join(endpoint.trim_start_matches('/'))
            .map_err(From::from)
    }

    fn has_api_key(&self) -> bool {
        self.api_key.token.is_some()
    }
}

impl api::Client for SpeedrunApiClient {
    fn rest(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<bytes::Bytes>, api::ApiError<Self::Error>> {
        let call = || -> Result<_, RestError> {
            self.api_key
                .set_auth_header(request.headers_mut().unwrap())?;
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request)?;

            let mut http_rsp = http::Response::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, val) in rsp.headers() {
                headers.insert(key, val.clone());
            }
            http_rsp.body(rsp.bytes()?).map_err(From::from)
        };
        call().map_err(api::ApiError::client)
    }
}

/// An asynchronous client for communicating with the Speedrun.com API
#[derive(Clone, Debug)]
pub struct SpeedrunApiClientAsync {
    client: AsyncHttpClient,
    rest_url: Url,
    api_key: Auth,
}

impl SpeedrunApiClientAsync {
    /// Create a new asynchronous Speedrun.com API client
    pub fn new() -> SpeedrunApiResult<Self> {
        Self::new_impl::<String>(None)
    }

    /// Create a new asynchronous Speedrun.com API client, with the provided API
    /// key.
    pub fn with_api_key<S>(api_key: S) -> SpeedrunApiResult<Self>
    where
        S: Into<String>,
    {
        Self::new_impl(Some(api_key.into()))
    }

    fn new_impl<S>(api_key: Option<S>) -> SpeedrunApiResult<Self>
    where
        S: Into<String>,
    {
        let rest_url = Url::parse(SPEEDRUN_API_BASE_URL)?;
        let client = AsyncHttpClient::new();
        let auth = Auth {
            token: api_key.map(Into::into),
        };
        let api = Self {
            client,
            rest_url,
            api_key: auth,
        };
        Ok(api)
    }

    /// Create a new Speedrun.com API builder.
    pub fn builder() -> SpeedrunApiBuilder {
        SpeedrunApiBuilder::new()
    }
}

impl api::RestClient for SpeedrunApiClientAsync {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        debug!("REST api call {}", endpoint);
        self.rest_url
            .join(endpoint.trim_start_matches('/'))
            .map_err(From::from)
    }

    fn has_api_key(&self) -> bool {
        self.api_key.token.is_some()
    }
}

#[async_trait]
impl api::AsyncClient for SpeedrunApiClientAsync {
    async fn rest_async(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<bytes::Bytes>, api::ApiError<Self::Error>> {
        let call = || async {
            self.api_key
                .set_auth_header(request.headers_mut().unwrap())?;
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request).await?;

            let mut http_rsp = http::Response::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, val) in rsp.headers() {
                headers.insert(key, val.clone());
            }
            http_rsp.body(rsp.bytes().await?).map_err(From::from)
        };
        call().map_err(api::ApiError::client).await
    }
}

/// Speedrun.com API client builder
#[derive(Debug, Default)]
pub struct SpeedrunApiBuilder {
    api_key: Option<String>,
}

impl SpeedrunApiBuilder {
    /// Create a new Speedrun.com API client builder.
    pub fn new() -> Self {
        SpeedrunApiBuilder::default()
    }

    /// Add an API key
    pub fn api_key<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.api_key = Some(value.into());
        self
    }

    /// Build a synchronous Speedrun.com API client.
    pub fn build(&self) -> SpeedrunApiResult<SpeedrunApiClient> {
        SpeedrunApiClient::new_impl(self.api_key.as_ref())
    }

    /// Build an asynchronous Speedrun.com API client.
    pub fn build_async(&self) -> SpeedrunApiResult<SpeedrunApiClientAsync> {
        SpeedrunApiClientAsync::new_impl(self.api_key.as_ref())
    }
}
