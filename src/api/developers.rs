use std::{borrow::Cow, fmt::Display};

use http::Method;
use serde::{Deserialize, Serialize};

use super::{endpoint::Endpoint, Direction, Pageable};

/// Sorting options for developers
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum DevelopersSorting {
    /// Sort alphanumerically by developer name (default)
    Name,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct DeveloperId<'a>(Cow<'a, str>);

impl<'a> DeveloperId<'a> {
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

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Developers {
    orderby: Option<DevelopersSorting>,
    direction: Option<Direction>,
}

#[derive(Debug, Builder, Clone)]
#[builder(setter(into, strip_option))]
pub struct Developer<'a> {
    id: DeveloperId<'a>,
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
