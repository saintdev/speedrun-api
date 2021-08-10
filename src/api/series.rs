use std::{borrow::Cow, collections::BTreeSet, fmt::Display};

use http::Method;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{
    developers::DeveloperId,
    endpoint::Endpoint,
    engines::EngineId,
    games::{Games, GamesBuilder, GamesBuilderError, GamesSorting},
    gametypes::GameTypeId,
    genres::GenreId,
    platforms::PlatformId,
    publishers::PublisherId,
    regions::RegionId,
    users::UserId,
    Direction, Pageable,
};

/// Embeds available for series.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SeriesEmbeds {
    Moderators,
}

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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SeriesId<'a>(Cow<'a, str>);

impl<'a> SeriesId<'a> {
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for SeriesId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl Display for SeriesId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct ListSeries<'a> {
    name: Option<Cow<'a, str>>,
    abbreviation: Option<Cow<'a, str>>,
    moderator: Option<UserId<'a>>,
    orderby: Option<SeriesSorting>,
    direction: Option<Direction>,
    #[builder(setter(name = "_embed"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<SeriesEmbeds>,
}

#[derive(Debug, Builder, Clone)]
#[builder(setter(into, strip_option))]
pub struct Series<'a> {
    id: SeriesId<'a>,
}

#[derive(Debug, Clone)]
pub struct SeriesGames<'a> {
    id: SeriesId<'a>,
    inner: Games<'a>,
}

#[derive(Default, Clone)]
pub struct SeriesGamesBuilder<'a> {
    id: Option<SeriesId<'a>>,
    inner: GamesBuilder<'a>,
}

impl<'a> ListSeries<'a> {
    pub fn builder() -> ListSeriesBuilder<'a> {
        ListSeriesBuilder::default()
    }
}

impl<'a> ListSeriesBuilder<'a> {
    pub fn embed(&mut self, embed: SeriesEmbeds) -> &mut Self {
        self.embed.get_or_insert_with(BTreeSet::new).insert(embed);
        self
    }

    pub fn embeds<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = SeriesEmbeds>,
    {
        self.embed.get_or_insert_with(BTreeSet::new).extend(iter);
        self
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
        S: Into<SeriesId<'a>>,
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
        S: Into<GameTypeId<'a>>,
    {
        self.inner.gametype(value);
        self
    }

    pub fn platform<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<PlatformId<'a>>,
    {
        self.inner.platform(value);
        self
    }

    pub fn region<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<RegionId<'a>>,
    {
        self.inner.region(value);
        self
    }

    pub fn genre<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<GenreId<'a>>,
    {
        self.inner.genre(value);
        self
    }

    pub fn engine<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<EngineId<'a>>,
    {
        self.inner.engine(value);
        self
    }

    pub fn developer<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<DeveloperId<'a>>,
    {
        self.inner.developer(value);
        self
    }

    pub fn publisher<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<PublisherId<'a>>,
    {
        self.inner.publisher(value);
        self
    }

    pub fn moderator<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<UserId<'a>>,
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

impl SeriesEmbeds {
    fn as_str(&self) -> &'static str {
        match self {
            SeriesEmbeds::Moderators => "moderators",
        }
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

impl From<&SeriesEmbeds> for &'static str {
    fn from(value: &SeriesEmbeds) -> Self {
        value.as_str()
    }
}

impl Pageable for ListSeries<'_> {}

impl Pageable for SeriesGames<'_> {}
