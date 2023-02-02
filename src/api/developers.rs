//! # Developers
//!
//! Endpoints available for developers

use std::{borrow::Cow, fmt::Display};

use http::Method;
use serde::{Deserialize, Serialize};

use super::{endpoint::Endpoint, error::BodyError, query_params::QueryParams, Direction, Pageable};

/// Sorting options for developers
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum DevelopersSorting {
    /// Sort alphanumerically by developer name (default)
    Name,
}

/// Represents a developer ID.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct DeveloperId<'a>(Cow<'a, str>);

impl<'a> DeveloperId<'a> {
    /// Create a new [`DeveloperId`]
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for DeveloperId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl Display for DeveloperId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// Retrieves a list of developers
#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Developers {
    #[doc = r"Sorting options for results."]
    orderby: Option<DevelopersSorting>,
    #[doc = r"Sort direction"]
    direction: Option<Direction>,
}

/// Retrieves a single developer identified by ID
#[derive(Debug, Builder, Clone)]
#[builder(setter(into, strip_option))]
pub struct Developer<'a> {
    #[doc = r"`ID` of the developer to retrieve"]
    id: DeveloperId<'a>,
}

impl Developers {
    /// Create a builder for this endpoint.
    pub fn builder() -> DevelopersBuilder {
        DevelopersBuilder::default()
    }
}

impl Developer<'_> {
    /// Create a builder for this endpoint.
    pub fn builder<'a>() -> DeveloperBuilder<'a> {
        DeveloperBuilder::default()
    }
}

impl Default for DevelopersSorting {
    fn default() -> Self {
        Self::Name
    }
}

impl Endpoint for Developers {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "/developers".into()
    }

    fn query_parameters(&self) -> Result<QueryParams<'_>, BodyError> {
        QueryParams::with(self)
    }
}

impl Endpoint for Developer<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/developers/{}", self.id).into()
    }
}

impl Pageable for Developers {}
