use std::convert::TryInto;

use async_trait::async_trait;
use futures::TryFutureExt;
use log::debug;
use reqwest::{blocking::Client, Client as AsyncClient};
use url::Url;

use crate::{
    api,
    error::{RestError, SpeedrunApiResult},
};

const SPEEDRUN_API_BASE_URL: &str = "https://www.speedrun.com/api/v1/";

#[derive(Clone, Debug)]
pub struct SpeedrunApiClient {
    client: Client,
    rest_url: Url,
}

impl SpeedrunApiClient {
    pub fn new() -> SpeedrunApiResult<Self> {
        let rest_url = Url::parse(SPEEDRUN_API_BASE_URL)?;
        let api = SpeedrunApiClient {
            client: Client::new(),
            rest_url,
        };

        Ok(api)
    }

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
}

impl api::Client for SpeedrunApiClient {
    fn rest(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<bytes::Bytes>, api::ApiError<Self::Error>> {
        let call = || -> Result<_, RestError> {
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

pub struct SpeedrunApiClientAsync {
    client: AsyncClient,
    rest_url: Url,
}

impl SpeedrunApiClientAsync {
    async fn new() -> SpeedrunApiResult<Self> {
        let rest_url = Url::parse(SPEEDRUN_API_BASE_URL)?;
        let client = AsyncClient::new();
        let api = Self { client, rest_url };
        Ok(api)
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
}

#[async_trait]
impl api::AsyncClient for SpeedrunApiClientAsync {
    async fn rest_async(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<bytes::Bytes>, api::ApiError<Self::Error>> {
        let call = || async {
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

pub struct SpeedrunApiBuilder {}

impl SpeedrunApiBuilder {
    pub fn new() -> Self {
        SpeedrunApiBuilder {}
    }

    pub fn build(&self) -> SpeedrunApiResult<SpeedrunApiClient> {
        SpeedrunApiClient::new()
    }

    pub async fn build_async(&self) -> SpeedrunApiResult<SpeedrunApiClientAsync> {
        SpeedrunApiClientAsync::new().await
    }
}

impl Default for SpeedrunApiBuilder {
    fn default() -> Self {
        Self::new()
    }
}
