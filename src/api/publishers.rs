use std::borrow::Cow;

use http::Method;
use serde::Serialize;

use super::{endpoint::Endpoint, Direction, Pageable};

/// Sorting options for publisher
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum PublishersSorting {
    /// Sort alphanumerically by publisher name (default)
    Name,
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Publishers {
    orderby: Option<PublishersSorting>,
    direction: Option<Direction>,
}

#[derive(Default, Debug, Builder, Clone)]
#[builder(default, setter(into, strip_option))]
pub struct Publisher<'a> {
    id: Cow<'a, str>,
}

impl Publishers {
    pub fn builder() -> PublishersBuilder {
        PublishersBuilder::default()
    }
}

impl<'a> Publisher<'a> {
    pub fn builder() -> PublisherBuilder<'a> {
        PublisherBuilder::default()
    }
}

impl Default for PublishersSorting {
    fn default() -> Self {
        Self::Name
    }
}

impl Endpoint for Publishers {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "/publishers".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Endpoint for Publisher<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/publishers/{}", self.id).into()
    }
}

impl Pageable for Publishers {}