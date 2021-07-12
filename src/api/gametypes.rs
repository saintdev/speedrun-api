use std::borrow::Cow;

use http::Method;
use serde::Serialize;

use super::{endpoint::Endpoint, Direction, Pageable};

/// Sorting options for game type
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum GameTypesSorting {
    /// Sort alphanumerically by game type name (default)
    Name,
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct GameTypes {
    orderby: Option<GameTypesSorting>,
    direction: Option<Direction>,
}

#[derive(Default, Debug, Builder, Clone)]
#[builder(default, setter(into, strip_option))]
pub struct GameType<'a> {
    id: Cow<'a, str>,
}

impl GameTypes {
    pub fn builder() -> GameTypesBuilder {
        GameTypesBuilder::default()
    }
}

impl<'a> GameType<'a> {
    pub fn builder() -> GameTypeBuilder<'a> {
        GameTypeBuilder::default()
    }
}

impl Default for GameTypesSorting {
    fn default() -> Self {
        Self::Name
    }
}

impl Endpoint for GameTypes {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "/gametypes".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Endpoint for GameType<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/gametypes/{}", self.id).into()
    }
}

impl Pageable for GameTypes {}
