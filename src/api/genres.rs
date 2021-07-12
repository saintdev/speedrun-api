use std::borrow::Cow;

use http::Method;
use serde::Serialize;

use super::{endpoint::Endpoint, Direction, Pageable};

/// Sorting options for genre
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum GenresSorting {
    /// Sort alphanumerically by genre name (default)
    Name,
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Genres {
    orderby: Option<GenresSorting>,
    direction: Option<Direction>,
}

#[derive(Default, Debug, Builder, Clone)]
#[builder(default, setter(into, strip_option))]
pub struct Genre<'a> {
    id: Cow<'a, str>,
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
