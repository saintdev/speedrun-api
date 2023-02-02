use async_trait::async_trait;
use futures::{stream::BoxStream, StreamExt, TryStreamExt};
use serde::de::DeserializeOwned;

use crate::types::Pagination;

use super::{
    endpoint::Endpoint,
    query::{AsyncQuery, Query},
    utils::{build_paged_request, deserialize_response},
    ApiError, AsyncClient, Client, RestClient,
};

// TODO: Use provided "next" link for pagination

/// Marker trait to indicate that an endpoint is pageable.
pub trait Pageable {}

/// Adapters specific to [`Pageable`] endpoints.
pub trait PagedEndpointExt<'a, E> {
    /// Create an Iterator over the results of the paginated endpoint.
    fn iter<T, C>(&'a self, client: &'a C) -> PagedIter<'a, E, C, T>
    where
        C: Client,
        T: DeserializeOwned;

    /// Retrieves a single page of results for the paginated endpoint.
    fn single_page(&'a self) -> SinglePageBuilder<'a, E>;

    /// Create an async Stream over the results of the paginated endpoint.
    fn stream<T, C>(&'a self, client: &'a C) -> BoxStream<'a, Result<T, ApiError<C::Error>>>
    where
        T: DeserializeOwned + Send + 'static,
        C: AsyncClient + Sync,
        E: Sync + Send;
}

/// Iterator type for the [`iter`] method on [`PagedEndpointExt`].
///
/// [`iter`]: PagedEndpointExt::iter
pub struct PagedIter<'a, E, C, T> {
    client: &'a C,
    state: SinglePage<'a, E>,
    last_page: bool,
    current_page: Vec<T>,
}

/// Builder for the [`SinglePage`] endpoint
#[derive(Debug)]
pub struct SinglePageBuilder<'a, E> {
    inner: &'a E,
    offset: Option<usize>,
    max: Option<usize>,
}

/// Represents a single page of elements.
#[derive(Debug)]
pub struct SinglePage<'a, E> {
    pub(crate) inner: &'a E,
    offset: usize,
    max: Option<usize>,
}

impl<'a, E, C, T> PagedIter<'a, E, C, T>
where
    E: Endpoint + Pageable,
{
    pub(crate) fn new(paged: &'a E, client: &'a C) -> Self {
        let state = SinglePage::<E>::builder(paged).offset(0).build();
        Self {
            client,
            state,
            last_page: false,
            current_page: Vec::new(),
        }
    }
}

impl<'a, E> SinglePageBuilder<'a, E>
where
    E: Pageable + Endpoint,
{
    /// Create a new [`SinglePageBuilder`].
    pub fn new(paged: &'a E) -> Self {
        Self {
            inner: paged,
            offset: None,
            max: None,
        }
    }

    /// Request set of elements beginning at `offset`
    pub fn offset<T>(mut self, value: T) -> Self
    where
        T: Into<Option<usize>>,
    {
        self.offset = value.into();
        self
    }

    /// Number of elements per request. Valid values are between 1 and 200.
    pub fn page_size<T>(mut self, value: T) -> Self
    where
        T: Into<Option<usize>>,
    {
        // TODO: Validate that value is between 1 and 200.
        self.max = value.into();
        self
    }

    /// Returns a [`SinglePage`] that can be querired for a set of elements.
    pub fn build(self) -> SinglePage<'a, E>
    where
        E: Pageable,
    {
        SinglePage {
            inner: self.inner,
            offset: self.offset.unwrap_or(0),
            max: self.max,
        }
    }
}

impl<'a, E> SinglePage<'a, E>
where
    E: Endpoint + Pageable,
{
    /// Create a builder for a [`SinglePage`]
    pub fn builder(paged: &'a E) -> SinglePageBuilder<'a, E> {
        SinglePageBuilder::new(paged)
    }

    pub(crate) fn page_url<C: RestClient>(
        &self,
        client: &C,
    ) -> Result<url::Url, ApiError<C::Error>> {
        let mut url = client.rest_endpoint(&self.inner.endpoint())?;
        self.inner.set_query_parameters(&mut url)?;
        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("offset", &format!("{}", &self.offset));
            if let Some(max) = self.max {
                pairs.append_pair("max", &format!("{max}"));
            }
        }
        Ok(url)
    }
}

#[async_trait]
impl<'a, T, C, E> AsyncQuery<(Vec<T>, Pagination), C> for SinglePage<'a, E>
where
    T: DeserializeOwned + Send + 'static,
    C: AsyncClient + Sync,
    E: Endpoint + Pageable + Sync,
{
    async fn query_async(&self, client: &C) -> Result<(Vec<T>, Pagination), ApiError<C::Error>> {
        let (req, data) = build_paged_request(self, client)?;

        let url = req.uri_ref().cloned().unwrap_or_default();

        let rsp = client.rest_async(req, data).await?;

        deserialize_response::<_, C>(rsp)
            .map(|value| (value.data, value.pagination.unwrap_or_default()))
            .map_err(|err| ApiError::from_http_response(err, url))
    }
}

impl<'a, E, C, T> Iterator for PagedIter<'a, E, C, T>
where
    E: Endpoint + Pageable,
    T: DeserializeOwned,
    C: Client,
{
    type Item = Result<T, ApiError<C::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_page.is_empty() {
            if self.last_page {
                return None;
            }
            self.current_page = match self.state.query(self.client) {
                Ok((data, _pagination)) => data,
                Err(err) => return Some(Err(err)),
            };
            self.state.offset += self.current_page.len();

            // FIXME: 20 may not always be correct.
            if self.current_page.len() < self.state.max.unwrap_or(20) {
                self.last_page = true;
            }
            self.current_page.reverse();
        }

        self.current_page.pop().map(Ok)
    }
}

impl<'a, E> PagedEndpointExt<'a, E> for E
where
    E: Endpoint + Pageable,
{
    fn iter<T, C>(&'a self, client: &'a C) -> PagedIter<'a, E, C, T>
    where
        C: Client,
        T: DeserializeOwned,
    {
        PagedIter::new(self, client)
    }

    fn single_page(&self) -> SinglePageBuilder<'_, E> {
        SinglePageBuilder::new(self)
    }

    fn stream<T, C>(&'a self, client: &'a C) -> BoxStream<'_, Result<T, ApiError<C::Error>>>
    where
        T: DeserializeOwned + Send + 'static,
        C: AsyncClient + Sync,
        E: Sync + Send,
    {
        futures::stream::try_unfold(Some(0), move |state| async move {
            let offset = if let Some(offset) = state {
                offset
            } else {
                return Ok(None);
            };
            let page = SinglePageBuilder::new(self).offset(offset).build();
            let (data, pagination) = page.query_async(client).await?;
            if data.is_empty() {
                Ok::<_, ApiError<C::Error>>(None)
            } else {
                let next_state = if data.len() < pagination.max {
                    None
                } else {
                    // TODO: Dynamic page size
                    Some(offset + pagination.max)
                };
                Ok(Some((
                    futures::stream::iter(data.into_iter().map(Ok)),
                    next_state,
                )))
            }
        })
        .try_flatten()
        .boxed()
    }
}

impl<'a, E, T, C> Query<(Vec<T>, Pagination), C> for SinglePage<'a, E>
where
    E: Endpoint + Pageable,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<(Vec<T>, Pagination), ApiError<C::Error>> {
        let (req, data) = build_paged_request(self, client)?;

        let url = req.uri_ref().cloned().unwrap_or_default();

        let rsp = client.rest(req, data)?;

        deserialize_response::<_, C>(rsp)
            .map(|value| (value.data, value.pagination.unwrap_or_default()))
            .map_err(|err| ApiError::from_http_response(err, url))
    }
}
