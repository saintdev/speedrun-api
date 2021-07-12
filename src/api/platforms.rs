use std::borrow::Cow;

use http::Method;
use serde::Serialize;

use super::{endpoint::Endpoint, Direction, Pageable};

/// Sorting options for platforms
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum PlatformsSorting {
    /// Sorts alphanumerically by the platform name (default)
    Name,
    /// Sorts by the year the platform was released
    Released,
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Platforms {
    orderby: Option<PlatformsSorting>,
    direction: Option<Direction>,
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Platform<'a> {
    id: Cow<'a, str>,
}

impl Platforms {
    pub fn builder() -> PlatformsBuilder {
        PlatformsBuilder::default()
    }
}

impl<'a> Platform<'a> {
    pub fn builder() -> PlatformBuilder<'a> {
        PlatformBuilder::default()
    }
}

impl Default for PlatformsSorting {
    fn default() -> Self {
        Self::Name
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
