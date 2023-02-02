//! # Publishers
//!
//! Endpoints available for publishers.
use std::{borrow::Cow, fmt::Display};

use serde::{Deserialize, Serialize};

use super::{endpoint::Endpoint, error::BodyError, query_params::QueryParams, Direction, Pageable};

/// Sorting options for publisher
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum PublishersSorting {
    /// Sort alphanumerically by publisher name (default)
    Name,
}

/// Represents a publisher ID.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct PublisherId<'a>(Cow<'a, str>);

impl<'a> PublisherId<'a> {
    /// Create a new [`PublisherId`].
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for PublisherId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl Display for PublisherId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// Retrieves a list of all publishers.
#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Publishers {
    #[doc = r"Sorting options for results."]
    orderby: Option<PublishersSorting>,
    #[doc = r"Sort direction"]
    direction: Option<Direction>,
}

/// Retrieves a single publisher by id.
#[derive(Debug, Builder, Clone)]
#[builder(setter(into, strip_option))]
pub struct Publisher<'a> {
    #[doc = r"`ID` for the publisher."]
    id: PublisherId<'a>,
}

impl Publishers {
    /// Create a builder for this endpoint.
    pub fn builder() -> PublishersBuilder {
        PublishersBuilder::default()
    }
}

impl Publisher<'_> {
    /// Create a builder for this endpoint.
    pub fn builder<'a>() -> PublisherBuilder<'a> {
        PublisherBuilder::default()
    }
}

impl Default for PublishersSorting {
    fn default() -> Self {
        Self::Name
    }
}

impl Endpoint for Publishers {
    fn endpoint(&self) -> Cow<'static, str> {
        "/publishers".into()
    }

    fn query_parameters(&self) -> Result<QueryParams<'_>, BodyError> {
        QueryParams::with(self)
    }
}

impl Endpoint for Publisher<'_> {
    fn endpoint(&self) -> Cow<'static, str> {
        format!("/publishers/{}", self.id).into()
    }
}

impl Pageable for Publishers {}
