use http::{header, request::Builder as RequestBuilder};
use serde::Serializer;

use super::{
    endpoint::Endpoint, query::url_to_http_uri, ApiError, Pageable, RestClient, SinglePage,
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

    endpoint.set_query_parameters(&mut url)?;

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
