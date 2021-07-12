use std::borrow::Cow;

use http::Method;
use serde::Serialize;

use super::{endpoint::Endpoint, Direction, Pageable};

/// Sorting options for developers
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum DevelopersSorting {
    /// Sort alphanumerically by developer name (default)
    Name,
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Developers {
    orderby: Option<DevelopersSorting>,
    direction: Option<Direction>,
}

#[derive(Default, Debug, Builder, Clone)]
#[builder(default, setter(into, strip_option))]
pub struct Developer<'a> {
    id: Cow<'a, str>,
}

impl Developers {
    pub fn builder() -> DevelopersBuilder {
        DevelopersBuilder::default()
    }
}

impl<'a> Developer<'a> {
    pub fn builder() -> DeveloperBuilder<'a> {
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

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
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
