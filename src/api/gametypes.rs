use std::{borrow::Cow, fmt::Display};

use http::Method;
use serde::{Deserialize, Serialize};

use super::{endpoint::Endpoint, Direction, Pageable};

/// Sorting options for game type
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum GameTypesSorting {
    /// Sort alphanumerically by game type name (default)
    Name,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GameTypeId<'a>(Cow<'a, str>);

impl<'a> GameTypeId<'a> {
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for GameTypeId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl Display for GameTypeId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct GameTypes {
    orderby: Option<GameTypesSorting>,
    direction: Option<Direction>,
}

#[derive(Debug, Builder, Clone)]
#[builder(setter(into, strip_option))]
pub struct GameType<'a> {
    id: GameTypeId<'a>,
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
