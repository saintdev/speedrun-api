//! # Platforms
//!
//! Endpoints available for platforms.
use std::{borrow::Cow, fmt::Display};

use http::Method;
use serde::{Deserialize, Serialize};

use super::{endpoint::Endpoint, Direction, Pageable};

/// Sorting options for platforms
#[derive(Default, Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum PlatformsSorting {
    /// Sorts alphanumerically by the platform name (default)
    #[default]
    Name,
    /// Sorts by the year the platform was released
    Released,
}

/// Represents a platform ID.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PlatformId<'a>(Cow<'a, str>);

impl<'a> PlatformId<'a> {
    /// Create a new [`PlatformId`].
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for PlatformId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl Display for PlatformId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// Retrieves a list of all platforms.
#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Platforms {
    #[doc = r"Sorting options for results."]
    orderby: Option<PlatformsSorting>,
    #[doc = r"Sort direction"]
    direction: Option<Direction>,
}

/// Retrieves a single platform by ID.
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Platform<'a> {
    #[doc = r"`ID` of the platform to retrieve."]
    id: PlatformId<'a>,
}

impl Platforms {
    /// Create a builder for this endpoint.
    pub fn builder() -> PlatformsBuilder {
        PlatformsBuilder::default()
    }
}

impl<'a> Platform<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> PlatformBuilder<'a> {
        PlatformBuilder::default()
    }
}

impl Endpoint for Platforms {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "/platforms".into()
    }

    fn query_parameters(&self) -> Result<std::borrow::Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Endpoint for Platform<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/platforms/{}", self.id).into()
    }
}

impl Pageable for Platforms {}
