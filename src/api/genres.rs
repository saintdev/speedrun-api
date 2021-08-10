use std::{borrow::Cow, fmt::Display};

use http::Method;
use serde::{Deserialize, Serialize};

use super::{endpoint::Endpoint, Direction, Pageable};

/// Sorting options for genre
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum GenresSorting {
    /// Sort alphanumerically by genre name (default)
    Name,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenreId<'a>(Cow<'a, str>);

impl<'a> GenreId<'a> {
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for GenreId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl Display for GenreId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Genres {
    orderby: Option<GenresSorting>,
    direction: Option<Direction>,
}

#[derive(Debug, Builder, Clone)]
#[builder(setter(into, strip_option))]
pub struct Genre<'a> {
    id: GenreId<'a>,
}

impl Genres {
    pub fn builder() -> GenresBuilder {
        GenresBuilder::default()
    }
}

impl<'a> Genre<'a> {
    pub fn builder() -> GenreBuilder<'a> {
        GenreBuilder::default()
    }
}

impl Default for GenresSorting {
    fn default() -> Self {
        Self::Name
    }
}

impl Endpoint for Genres {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "/genres".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Endpoint for Genre<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/genres/{}", self.id).into()
    }
}

impl Pageable for Genres {}
