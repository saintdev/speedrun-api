use std::borrow::Cow;

use async_trait::async_trait;
use http::Method;
use serde::de::DeserializeOwned;

use super::{
    error::BodyError,
    query::{AsyncQuery, Query},
    query_params::QueryParams,
    utils::{build_request, deserialize_response},
    ApiError, AsyncClient, Client,
};

pub trait Endpoint {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str>;

    fn query_parameters(&self) -> Result<QueryParams<'_>, BodyError> {
        Ok(QueryParams::default())
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
        let (req, data) = build_request(self, client)?;

        let url = req.uri_ref().cloned().unwrap_or_default();

        let rsp = client.rest(req, data)?;

        deserialize_response::<_, C>(rsp)
            .map(|value| value.data)
            .map_err(|err| ApiError::from_http_response(err, url))
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
        let (req, data) = build_request(self, client)?;
        let url = req.uri_ref().cloned().unwrap_or_default();

        let rsp = client.rest_async(req, data).await?;

        deserialize_response::<_, C>(rsp)
            .map(|value| value.data)
            .map_err(|err| ApiError::from_http_response(err, url))
    }
}
