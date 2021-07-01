use async_trait::async_trait;
use futures::{stream::Stream, TryStreamExt};
use http::{header, Request};
use serde::de::DeserializeOwned;

use super::{
    endpoint::Endpoint,
    query::{self, AsyncQuery, Query},
    ApiError, AsyncClient, Client, RestClient,
};

// TODO: Use provided "next" link for pagination

const DEFAULT_PAGE_SIZE: usize = 20;

/// Marker trait to indicate that an endpoint is pageable.
pub trait Pageable {}

/// Represents a paginated endpoint
#[derive(Debug)]
pub struct Paged<E> {
    pub(crate) endpoint: E,
}

impl<E> Paged<E>
where
    E: Endpoint + Pageable,
{
    /// Create an iterator over the results of the paginated endpoint.
    pub fn iter<'a, T, C>(&'a self, client: &'a C) -> PagedIter<'a, E, C, T>
    where
        T: DeserializeOwned,
        C: Client,
    {
        PagedIter::new(self, client)
    }

    /// Retrieve a single page of the paginated endpoint.
    pub fn single_page() -> SinglePageBuilder {
        SinglePageBuilder::default()
    }

    // Convert this to a wrapping type? PagedStream<E>
    /// Create a async stream over the results of the paginated endpoint.
    pub fn stream<'a, T, C>(
        &'a self,
        client: &'a C,
    ) -> impl Stream<Item = Result<T, ApiError<C::Error>>> + 'a
    where
        T: DeserializeOwned + Send + 'static,
        C: AsyncClient + Sync,
        E: Sync,
    {
        futures::stream::try_unfold(Some(0), move |state| async move {
            let offset = if let Some(offset) = state {
                offset
            } else {
                return Ok(None);
            };
            let page = SinglePageBuilder::default().offset(offset).build(self);
            let data = page.query_async(client).await?;
            if data.is_empty() {
                Ok::<_, ApiError<C::Error>>(None)
            } else {
                // FIXME: Dynamic page size
                let next_state = if data.len() < DEFAULT_PAGE_SIZE {
                    None
                } else {
                    // FIXME: Dynamic page size
                    Some(offset + DEFAULT_PAGE_SIZE)
                };
                Ok(Some((
                    futures::stream::iter(data.into_iter().map(Ok)),
                    next_state,
                )))
            }
        })
        .try_flatten()
    }
}

/// Iterator type for the `iter` method.
pub struct PagedIter<'a, E, C, T> {
    client: &'a C,
    state: SinglePage<'a, E>,
    last_page: bool,
    current_page: Vec<T>,
}

impl<'a, E, C, T> PagedIter<'a, E, C, T>
where
    E: Endpoint + Pageable,
{
    pub(crate) fn new(paged: &'a Paged<E>, client: &'a C) -> Self {
        let state = SinglePage::<E>::builder().offset(0).build(paged);
        Self {
            client,
            state,
            last_page: false,
            current_page: Vec::new(),
        }
    }
}

/// Builder for the `SinglePage` endpoint
#[derive(Debug)]
pub struct SinglePageBuilder {
    offset: Option<usize>,
    max: Option<usize>,
}

impl SinglePageBuilder {
    /// Request set of elements beginning at `offset`
    pub fn offset(&mut self, value: usize) -> &mut Self {
        self.offset = Some(value);
        self
    }

    /// Number of elements per request. Valid values are between 1 and 200.
    pub fn page_size<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Option<usize>>,
    {
        // TODO: Validate that value is between 1 and 200.
        self.max = value.into();
        self
    }

    /// Returns a `SinglePage` that can be querired for a set of elements.
    pub fn build<'a, E>(&self, paged: &'a Paged<E>) -> SinglePage<'a, E>
    where
        E: Pageable,
    {
        SinglePage {
            paged,
            offset: self.offset.unwrap_or(0),
            max: self.max,
        }
    }
}

impl Default for SinglePageBuilder {
    fn default() -> Self {
        Self {
            offset: None,
            max: None,
        }
    }
}

/// Represents a single page of elements.
#[derive(Debug)]
pub struct SinglePage<'a, E> {
    paged: &'a Paged<E>,
    offset: usize,
    max: Option<usize>,
}

impl<'a, E> SinglePage<'a, E>
where
    E: Endpoint + Pageable,
{
    /// Create a builder for a `SinglePage`
    pub fn builder() -> SinglePageBuilder {
        SinglePageBuilder::default()
    }

    fn page_url<C: RestClient>(&self, client: &C) -> Result<url::Url, ApiError<C::Error>> {
        let mut url = client.rest_endpoint(&self.paged.endpoint.endpoint())?;
        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("offset", &format!("{}", &self.offset));
            if let Some(max) = self.max {
                pairs.append_pair("max", &format!("{}", max));
            }
        }
        Ok(url)
    }
}

impl<'a, E, T, C> Query<Vec<T>, C> for SinglePage<'a, E>
where
    E: Endpoint + Pageable,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        let url = self.page_url(client)?;

        let body = self.paged.endpoint.body()?;

        let req = Request::builder()
            .method(self.paged.endpoint.method())
            .uri(query::url_to_http_uri(url));

        let (req, data) = if let Some((mime, data)) = body.as_ref() {
            let req = req.header(header::CONTENT_TYPE, *mime);
            (req, data.clone())
        } else {
            (req, Vec::new())
        };

        let rsp = client.rest(req, data)?;
        let status = rsp.status();

        let val = serde_json::from_slice(rsp.body())?;
        if !status.is_success() {
            return Err(ApiError::from_speedrun_api(val));
        }

        serde_json::from_value(val).map_err(ApiError::data_type::<Vec<T>>)
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
                Ok(data) => data,
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

#[async_trait]
impl<'a, T, C, E> AsyncQuery<Vec<T>, C> for SinglePage<'a, E>
where
    T: DeserializeOwned + Send + 'static,
    C: AsyncClient + Sync,
    E: Endpoint + Pageable + Sync,
{
    async fn query_async(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        let url = self.page_url(client)?;

        let body = self.paged.endpoint.body()?;

        let req = Request::builder()
            .method(self.paged.endpoint.method())
            .uri(query::url_to_http_uri(url));

        let (req, data) = if let Some((mime, data)) = body.as_ref() {
            let req = req.header(header::CONTENT_TYPE, *mime);
            (req, data.clone())
        } else {
            (req, Vec::new())
        };

        let rsp = client.rest_async(req, data).await?;
        let status = rsp.status();

        let val = serde_json::from_slice(rsp.body())?;
        if !status.is_success() {
            return Err(ApiError::from_speedrun_api(val));
        }

        serde_json::from_value(val).map_err(ApiError::data_type::<Vec<T>>)
    }
}
