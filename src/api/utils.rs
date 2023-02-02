use bytes::Bytes;
use http::{header, request::Builder as RequestBuilder};
use serde::{de::DeserializeOwned, Serializer};
use thiserror::Error;

use super::{
    endpoint::Endpoint, query::url_to_http_uri, ApiError, Pageable, RestClient, Root, SinglePage,
};

pub(crate) fn serialize_as_csv<S, T>(
    iter: impl IntoIterator<Item = T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    T: Into<&'static str>,
    S: Serializer,
{
    let out: Vec<_> = iter.into_iter().map(Into::into).collect();
    serializer.serialize_str(&out.join(","))
}

pub(crate) fn build_request<E, C>(
    endpoint: &E,
    client: &C,
) -> Result<(RequestBuilder, Vec<u8>), ApiError<C::Error>>
where
    E: Endpoint,
    C: RestClient,
{
    let url = client.rest_endpoint(&endpoint.endpoint())?;
    build_request_internal(url, endpoint, client)
}

pub(crate) fn build_paged_request<E, C>(
    page: &SinglePage<'_, E>,
    client: &C,
) -> Result<(RequestBuilder, Vec<u8>), ApiError<C::Error>>
where
    E: Endpoint + Pageable,
    C: RestClient,
{
    let url = page.page_url(client)?;
    let endpoint = page.inner;
    build_request_internal(url, endpoint, client)
}

pub(crate) fn build_request_internal<E, C>(
    mut url: url::Url,
    endpoint: &E,
    client: &C,
) -> Result<(RequestBuilder, Vec<u8>), ApiError<C::Error>>
where
    E: Endpoint,
    C: RestClient,
{
    if endpoint.requires_authentication() && !client.has_api_key() {
        return Err(ApiError::RequiresAuthentication);
    }

    endpoint.query_parameters()?.apply_to(&mut url);

    let req = RequestBuilder::new()
        .method(endpoint.method())
        .uri(url_to_http_uri(url));
    if let Some((mime, data)) = endpoint.body()? {
        let req = req.header(header::CONTENT_TYPE, mime);
        Ok((req, data))
    } else {
        Ok((req, Vec::new()))
    }
}

#[derive(Debug, Error)]
pub enum ResponseError {
    #[error("Parsing JSON: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("Deserializing value: {source}")]
    DataType {
        source: serde_json::Error,
        value: serde_json::Value,
        typ: &'static str,
    },
    #[error("HTTP error: {status}")]
    HttpStatus {
        value: serde_json::Value,
        status: http::StatusCode,
    },
}

pub(crate) fn deserialize_response<T, C>(
    rsp: http::Response<Bytes>,
) -> Result<Root<T>, ResponseError>
where
    T: DeserializeOwned,
    C: RestClient,
{
    let status = rsp.status();
    let value = serde_json::from_slice(rsp.body())?;
    if !status.is_success() {
        return Err(ResponseError::HttpStatus { value, status });
    }

    serde_json::from_value::<Root<T>>(value.clone()).map_err(|err| ResponseError::DataType {
        source: err,
        value,
        typ: std::any::type_name::<T>(),
    })
}
