use std::borrow::Cow;

use async_trait::async_trait;
use http::{header, Method, Request};
use log::debug;
use serde::de::DeserializeOwned;

use super::{
    common::Root,
    error::BodyError,
    query::{self, AsyncQuery, Query},
    ApiError, AsyncClient, Client,
};

pub trait Endpoint {
    fn method(&self) -> Method;
    fn endpoint(&self) -> Cow<'static, str>;
    fn set_query_parameters(&self, url: &mut url::Url) -> Result<(), BodyError> {
        let old_query: Vec<(String, String)> = url.query_pairs().into_owned().collect();
        debug!("old query: {:?}", old_query);
        let query = self.query_parameters()?;
        debug!("new query: {}", query);
        if !query.is_empty() {
            url.set_query(Some(query.as_ref()));
            if !old_query.is_empty() {
                url.query_pairs_mut().extend_pairs(old_query);
            }
        }
        Ok(())
    }
    fn query_parameters(&self) -> Result<Cow<'static, str>, BodyError> {
        Ok(Cow::default())
    }
    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        Ok(None)
    }

    //NOTE: Move this into a type/trait?
    /// If this endpoint requires a valid API key
    fn requires_authentication(&self) -> bool {
        false
    }
}

impl<E, T, C> Query<T, C> for E
where
    E: Endpoint,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<T, super::ApiError<C::Error>> {
        if self.requires_authentication() && !client.has_api_key() {
            return Err(ApiError::RequiresAuthentication);
        }
        let mut url = client.rest_endpoint(&self.endpoint())?;
        self.set_query_parameters(&mut url)?;

        let req = Request::builder()
            .method(self.method())
            .uri(query::url_to_http_uri(url));
        let (req, data) = if let Some((mime, data)) = self.body()? {
            let req = req.header(header::CONTENT_TYPE, mime);
            (req, data)
        } else {
            (req, Vec::new())
        };
        let rsp = client.rest(req, data)?;
        let status = rsp.status();
        let value = serde_json::from_slice(rsp.body())?;
        if !status.is_success() {
            return Err(ApiError::from_speedrun_api(value));
        }

        serde_json::from_value::<Root<T>>(value)
            .map(|root| root.data)
            .map_err(ApiError::data_type::<T>)
    }
}

#[async_trait]
impl<E, T, C> AsyncQuery<T, C> for E
where
    E: Endpoint + Sync,
    T: DeserializeOwned + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        if self.requires_authentication() && !client.has_api_key() {
            return Err(ApiError::RequiresAuthentication);
        }
        let mut url = client.rest_endpoint(&self.endpoint())?;
        self.set_query_parameters(&mut url)?;

        let req = Request::builder()
            .method(self.method())
            .uri(query::url_to_http_uri(url));
        let (req, data) = if let Some((mime, data)) = self.body()? {
            let req = req.header(header::CONTENT_TYPE, mime);
            (req, data)
        } else {
            (req, Vec::new())
        };

        let rsp = client.rest_async(req, data).await?;
        let status = rsp.status();
        let value = serde_json::from_slice(rsp.body())?;
        if !status.is_success() {
            return Err(ApiError::from_speedrun_api(value));
        }

        serde_json::from_value::<Root<T>>(value)
            .map(|value| value.data)
            .map_err(ApiError::data_type::<T>)
    }
}
