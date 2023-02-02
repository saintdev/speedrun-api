//! # Regions
//!
//! Endpoints available for regions.
use std::{borrow::Cow, fmt::Display};

use http::Method;
use serde::{Deserialize, Serialize};

use super::{endpoint::Endpoint, error::BodyError, query_params::QueryParams, Direction, Pageable};

/// Represents a region ID.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct RegionId<'a>(Cow<'a, str>);

impl<'a> RegionId<'a> {
    /// Create a new [`RegionId`].
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for RegionId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl Display for RegionId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// Retreives a list of all regions.
#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Regions {
    #[doc = r"Sort direction. Regions are currently only sorted alphanumerically by the region name."]
    direction: Option<Direction>,
}

/// Retrieves a single region.
#[derive(Debug, Builder, Clone)]
#[builder(setter(into, strip_option))]
pub struct Region<'a> {
    #[doc = r"`ID` of the region."]
    id: RegionId<'a>,
}

impl Regions {
    /// Create a builder for this endpoint.
    pub fn builder() -> RegionsBuilder {
        RegionsBuilder::default()
    }
}

impl Region<'_> {
    /// Create a builder for this endpoint.
    pub fn builder<'a>() -> RegionBuilder<'a> {
        RegionBuilder::default()
    }
}

impl Endpoint for Regions {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "/regions".into()
    }

    fn query_parameters(&self) -> Result<QueryParams<'_>, BodyError> {
        QueryParams::with(self)
    }
}

impl Endpoint for Region<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/regions/{}", self.id).into()
    }
}

impl Pageable for Regions {}
