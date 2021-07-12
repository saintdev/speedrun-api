use std::borrow::Cow;

use http::Method;
use serde::Serialize;
use thiserror::Error;

use super::{
    endpoint::Endpoint,
    games::{Games, GamesBuilder, GamesBuilderError, GamesSorting},
    Direction, Pageable,
};

/// Sorting options for game series
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum SeriesSorting {
    /// Sorts alphanumerically by the international name (default)
    #[serde(rename = "name.int")]
    NameInternational,
    /// Sorts alphanumerically by the Japanese name
    #[serde(rename = "name.jap")]
    NameJapanese,
    /// Sorts alphanumerically by the abbreviation
    Abbreviation,
    /// Sorts by the date the series was added to speedrun.com
    Created,
}

#[derive(Debug, Error)]
pub enum SeriesGamesBuilderError {
    #[error("{0} must be initialized")]
    UninitializedField(&'static str),
    #[error(transparent)]
    Inner(#[from] GamesBuilderError),
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct ListSeries<'a> {
    name: Option<Cow<'a, str>>,
    abbreviation: Option<Cow<'a, str>>,
    moderator: Option<Cow<'a, str>>,
    orderby: Option<SeriesSorting>,
    direction: Option<Direction>,
}

#[derive(Default, Debug, Builder, Clone)]
#[builder(default, setter(into, strip_option))]
pub struct Series<'a> {
    id: Cow<'a, str>,
}

#[derive(Default, Debug, Clone)]
pub struct SeriesGames<'a> {
    id: Cow<'a, str>,
    inner: Games<'a>,
}

#[derive(Default, Clone)]
pub struct SeriesGamesBuilder<'a> {
    id: Option<Cow<'a, str>>,
    inner: GamesBuilder<'a>,
}

impl<'a> ListSeries<'a> {
    pub fn builder() -> ListSeriesBuilder<'a> {
        ListSeriesBuilder::default()
    }
}

impl<'a> Series<'a> {
    pub fn builder() -> SeriesBuilder<'a> {
        SeriesBuilder::default()
    }
}

impl<'a> SeriesGames<'a> {
    pub fn builder() -> SeriesGamesBuilder<'a> {
        SeriesGamesBuilder::default()
    }
}

impl<'a> SeriesGamesBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the series games builder's id.
    pub fn id<S>(&mut self, id: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.id = Some(id.into());
        self
    }

    pub fn name<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.inner.name(value);
        self
    }

    pub fn abbreviation<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.inner.abbreviation(value);
        self
    }

    pub fn released(&mut self, value: i64) -> &mut Self {
        self.inner.released(value);
        self
    }

    pub fn gametype<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.inner.gametype(value);
        self
    }

    pub fn platform<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.inner.platform(value);
        self
    }

    pub fn region<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.inner.region(value);
        self
    }

    pub fn genre<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.inner.genre(value);
        self
    }

    pub fn engine<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.inner.engine(value);
        self
    }

    pub fn developer<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.inner.developer(value);
        self
    }

    pub fn publisher<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.inner.publisher(value);
        self
    }

    pub fn moderator<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.inner.moderator(value);
        self
    }

    pub fn bulk(&mut self, value: bool) -> &mut Self {
        self.inner.bulk(value);
        self
    }

    pub fn orderby(&mut self, value: GamesSorting) -> &mut Self {
        self.inner.orderby(value);
        self
    }

    pub fn direction(&mut self, value: Direction) -> &mut Self {
        self.inner.direction(value);
        self
    }

    pub fn build(&self) -> Result<SeriesGames<'a>, SeriesGamesBuilderError> {
        let inner = self.inner.build()?;
        Ok(SeriesGames {
            id: self
                .id
                .as_ref()
                .cloned()
                .ok_or(SeriesGamesBuilderError::UninitializedField("id"))?,
            inner,
        })
    }
}

impl Default for SeriesSorting {
    fn default() -> Self {
        Self::NameInternational
    }
}

impl Endpoint for ListSeries<'_> {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "/series".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Endpoint for Series<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/series/{}", self.id).into()
    }
}

impl Endpoint for SeriesGames<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/series/{}/games", self.id).into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(&self.inner)?.into())
    }
}

impl Pageable for ListSeries<'_> {}

impl Pageable for SeriesGames<'_> {}