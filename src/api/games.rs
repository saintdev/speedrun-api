//! # Games
//!
//! Endpoints available for games

use std::{borrow::Cow, collections::BTreeSet, fmt::Display};

use http::Method;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{
    categories::CategoryEmbeds, developers::DeveloperId, endpoint::Endpoint, engines::EngineId,
    error::BodyError, gametypes::GameTypeId, genres::GenreId, leaderboards::LeaderboardEmbeds,
    platforms::PlatformId, publishers::PublisherId, regions::RegionId, users::UserId,
    CategoriesSorting, Direction, Pageable, VariablesSorting,
};

/// Embeds available for games
///
/// NOTE: Embeds can be nested. That is not handled by this API.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameEmbeds {
    /// Embed all levels defined for the game.
    Levels,
    /// Embed all categories defined for the game.
    Categories,
    /// Embed moderators as full user resources.
    Moderators,
    /// Embed all assigned gametypes.
    Gametypes,
    /// Embed all assigned platforms.
    Platforms,
    /// Embed all assigned regions.
    Regions,
    /// Embed all assigned genres.
    Genres,
    /// Embed all assigned engines.
    Engines,
    /// Embed all assigned developers.
    Developers,
    /// Embed all assigned publishers.
    Publishers,
    /// Embed all variables defined for the game.
    Variables,
}

/// Sorting options for games
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum GamesSorting {
    /// Sorts alphanumerically by the international name (default)
    #[serde(rename = "name.int")]
    NameInternational,
    /// Sorts alphanumerically by the Japanese name
    #[serde(rename = "name.jap")]
    NameJapanese,
    /// Sorts alphanumerically by the abbreviation
    Abbreviation,
    /// Sorts by the release date
    Released,
    /// Sorts by the date the game was added to speedrun.com
    Created,
    /// Sorts by string similarity. *Only available when searching games by
    /// name* (default when searching by name)
    Similarity,
}

// Does this belong here?
/// Sorting options for levels
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum LevelsSorting {
    /// Sorts alphanumerically by the level name
    Name,
    /// Sorts by the order defined by the game moderator (default)
    Pos,
}

// Does this belong here?
/// Type of leaderboard to return.
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum LeaderboardScope {
    /// Only return full-game categories.
    FullGame,
    /// Only return individual levels.
    Levels,
    /// Return all (default)
    All,
}

/// Error type for GameDerivedGamesBuilder
#[derive(Debug, Error)]
pub enum GameDerivedGamesBuilderError {
    /// Uninitialized Field
    #[error("{0} must be initialized")]
    UninitializedField(&'static str),
    /// Error from the inner GamesBuilder
    #[error(transparent)]
    Inner(#[from] GamesBuilderError),
}

/// Represents a game ID
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct GameId<'a>(Cow<'a, str>);

impl<'a> GameId<'a> {
    /// Create a new [`GameId`]
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for GameId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl Display for GameId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// Retrievs a lists of all games.
#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Games<'a> {
    #[doc = r"Performs a fuzzy search across game names and abbreviations."]
    name: Option<Cow<'a, str>>,
    #[doc = r"Perform an exact-match search for this abbreviation."]
    abbreviation: Option<Cow<'a, str>>,
    #[doc = r"Restrict results to games released in the given year."]
    released: Option<i64>,
    #[doc = r"Restrict results to the given game type."]
    gametype: Option<GameTypeId<'a>>,
    #[doc = r"Restrict results to the given platform."]
    platform: Option<PlatformId<'a>>,
    #[doc = r"Restrict results to the given region."]
    region: Option<RegionId<'a>>,
    #[doc = r"Restrict results to the given genre."]
    genre: Option<GenreId<'a>>,
    #[doc = r"Restrict results to the given engine."]
    engine: Option<EngineId<'a>>,
    #[doc = r"Restrict results to the given developer."]
    developer: Option<DeveloperId<'a>>,
    #[doc = r"Restrict results to the given publisher."]
    publisher: Option<PublisherId<'a>>,
    #[doc = r"Only return games moderated by the given user."]
    moderator: Option<UserId<'a>>,
    #[doc = r"Enable bulk access."]
    #[serde(rename = "_bulk")]
    bulk: Option<bool>,
    #[doc = r"Sorting options for results."]
    orderby: Option<GamesSorting>,
    #[doc = r"Sort direction."]
    direction: Option<Direction>,
    #[builder(setter(name = "_embed"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<GameEmbeds>,
}

/// Retrieves a single game, identified by ID.
#[derive(Debug, Builder, Clone)]
#[builder(setter(into, strip_option))]
pub struct Game<'a> {
    #[doc = r"`ID` of the game."]
    id: GameId<'a>,
}

/// Retrieve all categories for the given game.
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct GameCategories<'a> {
    #[doc = r"`ID` of the game to retrieve categories for."]
    #[serde(skip)]
    id: GameId<'a>,
    #[doc = r"Filter miscellaneous categories."]
    #[builder(default)]
    miscellaneous: Option<bool>,
    #[doc = r"Sorting options for results."]
    #[builder(default)]
    orderby: Option<CategoriesSorting>,
    #[doc = r"Sort direction."]
    #[builder(default)]
    direction: Option<Direction>,
    #[builder(setter(name = "_embed"), private, default)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<CategoryEmbeds>,
}

impl<'a> GameCategoriesBuilder<'a> {
    /// Add an embedded resource to this result
    pub fn embed(&mut self, embed: CategoryEmbeds) -> &mut Self {
        self.embed.get_or_insert_with(BTreeSet::new).insert(embed);
        self
    }

    /// Add multiple embedded resources to this result
    pub fn embeds<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = CategoryEmbeds>,
    {
        self.embed.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }
}

/// Retrieves all levels for the given game.
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct GameLevels<'a> {
    #[doc = r"`ID` of the game to retrieve levels for."]
    #[serde(skip)]
    id: GameId<'a>,
    #[doc = r"Sorting options for results."]
    #[builder(default)]
    orderby: Option<LevelsSorting>,
    #[doc = r"Sort direction."]
    #[builder(default)]
    direction: Option<Direction>,
}

/// Retrieves all variables for the given game.
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct GameVariables<'a> {
    #[doc = r"`ID` of the game to retrieve variables for."]
    #[serde(skip)]
    id: GameId<'a>,
    #[doc = r"Sorting options for results."]
    #[builder(default)]
    orderby: Option<VariablesSorting>,
    #[doc = r"Sort direction."]
    #[builder(default)]
    direction: Option<Direction>,
}

/// Builder for [`GameDerivedGames`].
#[derive(Default, Clone)]
pub struct GameDerivedGamesBuilder<'a> {
    id: Option<GameId<'a>>,
    inner: GamesBuilder<'a>,
}

/// Retrieves all games that have the given game as their base game.
#[derive(Debug, Clone)]
pub struct GameDerivedGames<'a> {
    id: GameId<'a>,
    inner: Games<'a>,
}

/// Retrieves all records (top 3 places) for every (category/level) combonation
/// of the given game.
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct GameRecords<'a> {
    #[doc = r"`ID` of the game to retrieve records for."]
    id: GameId<'a>,
    #[doc = r"Return the `top` *places* (this can result in more than `top` runs!). Defaults to 3."]
    #[builder(default)]
    top: Option<i64>,
    #[doc = r"When set to [`LeaderboardScope::FullGame`], only full-game categories will be included. When set to [`LeaderboardScope::Levels`] only individual levels are returned. Defaults to [`LeaderboardScope::All`]."]
    #[builder(default)]
    scope: Option<LeaderboardScope>,
    #[doc = r"When `false`, miscellaneous categories will not be included in the results."]
    #[builder(default)]
    miscellaneous: Option<bool>,
    #[doc = r"When `true`, empty leaderboards will not be included in the results."]
    #[builder(default)]
    skip_empty: Option<bool>,
    #[builder(setter(name = "_embed"), private, default)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<LeaderboardEmbeds>,
}

impl<'a> Games<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> GamesBuilder<'a> {
        GamesBuilder::default()
    }
}

impl<'a> GamesBuilder<'a> {
    /// Add an embedded resource to this result.
    pub fn embed(&mut self, embed: GameEmbeds) -> &mut Self {
        self.embed.get_or_insert_with(BTreeSet::new).insert(embed);
        self
    }

    /// Add multiple embedded resources to this result.
    pub fn embeds<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = GameEmbeds>,
    {
        self.embed.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }
}

impl<'a> Game<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> GameBuilder<'a> {
        GameBuilder::default()
    }
}

impl<'a> GameCategories<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> GameCategoriesBuilder<'a> {
        GameCategoriesBuilder::default()
    }
}

impl<'a> GameLevels<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> GameLevelsBuilder<'a> {
        GameLevelsBuilder::default()
    }
}

impl<'a> GameVariables<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> GameVariablesBuilder<'a> {
        GameVariablesBuilder::default()
    }
}

impl<'a> GameDerivedGamesBuilder<'a> {
    /// `ID` of the base game to retrieve games for.
    pub fn id<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<GameId<'a>>,
    {
        self.id = Some(value.into());
        self
    }

    /// Performs a fuzzy search across game names and abbreviations.
    pub fn name<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.inner.name(value);
        self
    }

    /// Perform an exact-match search for this abbreviation.
    pub fn abbreviation<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.inner.abbreviation(value);
        self
    }

    /// Restrict results to games released in the given year.
    pub fn released<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<i64>,
    {
        self.inner.released(value);
        self
    }

    /// Restrict results to the given game type.
    pub fn gametype<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<GameTypeId<'a>>,
    {
        self.inner.gametype(value);
        self
    }

    /// Restrict results to the given platform.
    pub fn platform<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<PlatformId<'a>>,
    {
        self.inner.platform(value);
        self
    }

    /// Restrict results to the given region.
    pub fn region<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<RegionId<'a>>,
    {
        self.inner.region(value);
        self
    }

    /// Restrict results to the given genre.
    pub fn genre<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<GenreId<'a>>,
    {
        self.inner.genre(value);
        self
    }

    /// Restrict results to the given engine.
    pub fn engine<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<EngineId<'a>>,
    {
        self.inner.engine(value);
        self
    }

    /// Restrict results to the given developer.
    pub fn developer<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<DeveloperId<'a>>,
    {
        self.inner.developer(value);
        self
    }

    /// Restrict results to the given publisher.
    pub fn publisher<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<PublisherId<'a>>,
    {
        self.inner.publisher(value);
        self
    }

    /// Only return games moderated by the given user.
    pub fn moderator<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<UserId<'a>>,
    {
        self.inner.moderator(value);
        self
    }

    /// Enable bulk access.
    pub fn bulk<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<bool>,
    {
        self.inner.bulk(value);
        self
    }

    /// Sorting options for results.
    pub fn orderby<V>(&mut self, value: V) -> &mut Self
    where
        V: Into<GamesSorting>,
    {
        self.inner.orderby(value);
        self
    }

    /// Sort direction.
    pub fn direction<V>(&mut self, value: V) -> &mut Self
    where
        V: Into<Direction>,
    {
        self.inner.direction(value);
        self
    }

    /// Builds a new [`GameDerivedGames`].
    ///
    /// # Errors
    ///
    /// If a required field has not been initialized.
    pub fn build(&self) -> Result<GameDerivedGames<'a>, GameDerivedGamesBuilderError> {
        let inner = self.inner.build()?;
        Ok(GameDerivedGames {
            id: self
                .id
                .as_ref()
                .cloned()
                .ok_or(GameDerivedGamesBuilderError::UninitializedField("id"))?,
            inner,
        })
    }
}

impl<'a> GameDerivedGames<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> GameDerivedGamesBuilder<'a> {
        GameDerivedGamesBuilder::default()
    }
}

impl<'a> GameRecords<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> GameRecordsBuilder<'a> {
        GameRecordsBuilder::default()
    }
}

impl<'a> GameRecordsBuilder<'a> {
    /// Add an embedded resource to this result.
    pub fn embed(&mut self, embed: LeaderboardEmbeds) -> &mut Self {
        self.embed.get_or_insert_with(BTreeSet::new).insert(embed);
        self
    }

    /// Add multiple embedded resources to this result.
    pub fn embeds<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = LeaderboardEmbeds>,
    {
        self.embed.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }
}

impl GameEmbeds {
    fn as_str(&self) -> &'static str {
        match self {
            GameEmbeds::Levels => "levels",
            GameEmbeds::Categories => "categories",
            GameEmbeds::Moderators => "moderators",
            GameEmbeds::Gametypes => "gametypes",
            GameEmbeds::Platforms => "platforms",
            GameEmbeds::Regions => "regions",
            GameEmbeds::Genres => "genres",
            GameEmbeds::Engines => "engines",
            GameEmbeds::Developers => "developers",
            GameEmbeds::Publishers => "publishers",
            GameEmbeds::Variables => "variables",
        }
    }
}

impl Default for LevelsSorting {
    fn default() -> Self {
        Self::Pos
    }
}

impl Endpoint for Games<'_> {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "games".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Endpoint for Game<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/games/{}", self.id).into()
    }
}

impl Endpoint for GameCategories<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/games/{}/categories", self.id).into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Endpoint for GameLevels<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/games/{}/levels", self.id).into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Endpoint for GameVariables<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/games/{}/variables", self.id).into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Endpoint for GameDerivedGames<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/games/{}/derived-games", self.id).into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, BodyError> {
        Ok(serde_urlencoded::to_string(&self.inner)?.into())
    }
}

impl Endpoint for GameRecords<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/games/{}/records", self.id).into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl From<&GameEmbeds> for &'static str {
    fn from(value: &GameEmbeds) -> Self {
        value.as_str()
    }
}

impl Pageable for GameDerivedGames<'_> {}

impl Pageable for Games<'_> {}

impl Pageable for GameRecords<'_> {}
