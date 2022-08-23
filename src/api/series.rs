//! # Series
//!
//! Endpoints available for series.
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
    /// Embed moderators a full user resources.
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

/// Error type for [`SeriesGameBuilder`]
#[derive(Debug, Error)]
pub enum SeriesGamesBuilderError {
    /// Uninitialized field
    #[error("{0} must be initialized")]
    UninitializedField(&'static str),
    /// Error from the inner type
    #[error(transparent)]
    Inner(#[from] GamesBuilderError),
}

/// Represents a series ID
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct SeriesId<'a>(Cow<'a, str>);

impl<'a> SeriesId<'a> {
    /// Create a new [`SeriesId`]
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

/// Retrieves a list of all series
#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct ListSeries<'a> {
    #[doc = r"When given, performs a fuzzy search across all series names and abbreviations."]
    name: Option<Cow<'a, str>>,
    #[doc = r"When given, performs an exact-match search for `abbreviation`."]
    abbreviation: Option<Cow<'a, str>>,
    #[doc = r"When given, only return series moderated by [`UserId`]"]
    moderator: Option<UserId<'a>>,
    #[doc = r"Sorting options for results."]
    orderby: Option<SeriesSorting>,
    #[doc = r"Sort direction"]
    direction: Option<Direction>,
    #[builder(setter(name = "_embed"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<SeriesEmbeds>,
}

/// Retrieves a single series
#[derive(Debug, Builder, Clone)]
#[builder(setter(into, strip_option))]
pub struct Series<'a> {
    #[doc = r"Series ID or abbreviation"]
    id: SeriesId<'a>,
}

/// Retrieves all games for a given series
#[derive(Debug, Clone)]
pub struct SeriesGames<'a> {
    id: SeriesId<'a>,
    inner: Games<'a>,
}

/// Builder for [`SeriesGames`]
#[derive(Default, Clone)]
pub struct SeriesGamesBuilder<'a> {
    /// Series ID or abbreviation
    id: Option<SeriesId<'a>>,
    inner: GamesBuilder<'a>,
}

impl<'a> ListSeries<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> ListSeriesBuilder<'a> {
        ListSeriesBuilder::default()
    }
}

impl<'a> ListSeriesBuilder<'a> {
    /// Add an embedded resource to this result
    pub fn embed(&mut self, embed: SeriesEmbeds) -> &mut Self {
        self.embed.get_or_insert_with(BTreeSet::new).insert(embed);
        self
    }

    /// Add multiple embedded resources to this result
    pub fn embeds<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = SeriesEmbeds>,
    {
        self.embed.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }
}

impl<'a> Series<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> SeriesBuilder<'a> {
        SeriesBuilder::default()
    }
}

impl<'a> SeriesGames<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> SeriesGamesBuilder<'a> {
        SeriesGamesBuilder::default()
    }
}

impl<'a> SeriesGamesBuilder<'a> {
    /// Create a new [`SeriesGamesBuilder`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Series ID or abbreviation
    pub fn id<S>(&mut self, id: S) -> &mut Self
    where
        S: Into<SeriesId<'a>>,
    {
        self.id = Some(id.into());
        self
    }

    #[doc = r"Performs a fuzzy search across game names and abbreviations."]
    pub fn name<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.inner.name(value);
        self
    }

    #[doc = r"Perform an exact-match search for this abbreviation."]
    pub fn abbreviation<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.inner.abbreviation(value);
        self
    }

    #[doc = r"Restrict results to games released in the given year."]
    pub fn released(&mut self, value: i64) -> &mut Self {
        self.inner.released(value);
        self
    }

    #[doc = r"Restrict results to the given game type."]
    pub fn gametype<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<GameTypeId<'a>>,
    {
        self.inner.gametype(value);
        self
    }

    #[doc = r"Restrict results to the given platform."]
    pub fn platform<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<PlatformId<'a>>,
    {
        self.inner.platform(value);
        self
    }

    #[doc = r"Restrict results to the given region."]
    pub fn region<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<RegionId<'a>>,
    {
        self.inner.region(value);
        self
    }

    #[doc = r"Restrict results to the given genre."]
    pub fn genre<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<GenreId<'a>>,
    {
        self.inner.genre(value);
        self
    }

    #[doc = r"Restrict results to the given engine."]
    pub fn engine<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<EngineId<'a>>,
    {
        self.inner.engine(value);
        self
    }

    #[doc = r"Restrict results to the given developer."]
    pub fn developer<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<DeveloperId<'a>>,
    {
        self.inner.developer(value);
        self
    }

    #[doc = r"Restrict results to the given publisher."]
    pub fn publisher<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<PublisherId<'a>>,
    {
        self.inner.publisher(value);
        self
    }

    #[doc = r"Only return games moderated by the given user."]
    pub fn moderator<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<UserId<'a>>,
    {
        self.inner.moderator(value);
        self
    }

    #[doc = r"Enable bulk access."]
    pub fn bulk(&mut self, value: bool) -> &mut Self {
        self.inner.bulk(value);
        self
    }

    #[doc = r"Sorting options for results."]
    pub fn orderby(&mut self, value: GamesSorting) -> &mut Self {
        self.inner.orderby(value);
        self
    }

    #[doc = r"Sort direction."]
    pub fn direction(&mut self, value: Direction) -> &mut Self {
        self.inner.direction(value);
        self
    }

    /// Builds a new [`SeriesGames`]
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
