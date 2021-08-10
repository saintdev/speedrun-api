use std::{borrow::Cow, collections::BTreeSet, fmt::Display};

use http::Method;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{
    developers::DeveloperId, endpoint::Endpoint, engines::EngineId, error::BodyError,
    gametypes::GameTypeId, genres::GenreId, leaderboards::LeaderboardEmbeds, platforms::PlatformId,
    publishers::PublisherId, regions::RegionId, users::UserId, CategoriesSorting, Direction,
    Pageable, VariablesSorting,
};

/// Embeds available for games
///
/// NOTE: Embeds can be nested. That is not handled by this API.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameEmbeds {
    Levels,
    Categories,
    Moderators,
    Gametypes,
    Platforms,
    Regions,
    Genres,
    Engines,
    Developers,
    Publishers,
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
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum LeaderboardScope {
    FullGame,
    Levels,
    All,
}

#[derive(Debug, Error)]
pub enum GameDerivedGamesBuilderError {
    #[error("{0} must be initialized")]
    UninitializedField(&'static str),
    #[error(transparent)]
    Inner(#[from] GamesBuilderError),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GameId<'a>(Cow<'a, str>);

impl<'a> GameId<'a> {
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

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Games<'a> {
    name: Option<Cow<'a, str>>,
    abbreviation: Option<Cow<'a, str>>,
    released: Option<i64>,
    gametype: Option<GameTypeId<'a>>,
    platform: Option<PlatformId<'a>>,
    region: Option<RegionId<'a>>,
    genre: Option<GenreId<'a>>,
    engine: Option<EngineId<'a>>,
    developer: Option<DeveloperId<'a>>,
    publisher: Option<PublisherId<'a>>,
    moderator: Option<UserId<'a>>,
    #[serde(rename = "_bulk")]
    bulk: Option<bool>,
    orderby: Option<GamesSorting>,
    direction: Option<Direction>,
    #[builder(setter(name = "_embed"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<GameEmbeds>,
}

#[derive(Debug, Builder, Clone)]
#[builder(setter(into, strip_option))]
pub struct Game<'a> {
    id: GameId<'a>,
}

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct GameCategories<'a> {
    #[serde(skip)]
    id: GameId<'a>,
    #[builder(default)]
    miscellaneous: Option<bool>,
    #[builder(default)]
    orderby: Option<CategoriesSorting>,
    #[builder(default)]
    direction: Option<Direction>,
}

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct GameLevels<'a> {
    #[serde(skip)]
    id: GameId<'a>,
    #[builder(default)]
    orderby: Option<LevelsSorting>,
    #[builder(default)]
    direction: Option<Direction>,
}

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct GameVariables<'a> {
    #[serde(skip)]
    id: GameId<'a>,
    #[builder(default)]
    orderby: Option<VariablesSorting>,
    #[builder(default)]
    direction: Option<Direction>,
}

#[derive(Default, Clone)]
pub struct GameDerivedGamesBuilder<'a> {
    id: Option<GameId<'a>>,
    inner: GamesBuilder<'a>,
}

#[derive(Debug, Clone)]
pub struct GameDerivedGames<'a> {
    id: GameId<'a>,
    inner: Games<'a>,
}

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct GameRecords<'a> {
    id: GameId<'a>,
    #[builder(default)]
    top: Option<i64>,
    #[builder(default)]
    scope: Option<LeaderboardScope>,
    #[builder(default)]
    miscellaneous: Option<bool>,
    #[builder(default)]
    skip_empty: Option<bool>,
    #[builder(setter(name = "_embed"), private, default)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<LeaderboardEmbeds>,
}

impl<'a> Games<'a> {
    pub fn builder() -> GamesBuilder<'a> {
        GamesBuilder::default()
    }
}

impl<'a> GamesBuilder<'a> {
    pub fn embed(&mut self, embed: GameEmbeds) -> &mut Self {
        self.embed.get_or_insert_with(BTreeSet::new).insert(embed);
        self
    }

    pub fn embeds<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = GameEmbeds>,
    {
        self.embed.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }
}

impl<'a> Game<'a> {
    pub fn builder() -> GameBuilder<'a> {
        GameBuilder::default()
    }
}

impl<'a> GameCategories<'a> {
    pub fn builder() -> GameCategoriesBuilder<'a> {
        GameCategoriesBuilder::default()
    }
}

impl<'a> GameLevels<'a> {
    pub fn builder() -> GameLevelsBuilder<'a> {
        GameLevelsBuilder::default()
    }
}

impl<'a> GameVariables<'a> {
    pub fn builder() -> GameVariablesBuilder<'a> {
        GameVariablesBuilder::default()
    }
}

impl<'a> GameDerivedGamesBuilder<'a> {
    pub fn id<S>(&mut self, value: S) -> &mut Self
    where
        S: Into<GameId<'a>>,
    {
        self.id = Some(value.into());
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
    pub fn released<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<i64>,
    {
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
    pub fn bulk<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<bool>,
    {
        self.inner.bulk(value);
        self
    }
    pub fn orderby<V>(&mut self, value: V) -> &mut Self
    where
        V: Into<GamesSorting>,
    {
        self.inner.orderby(value);
        self
    }
    pub fn direction<V>(&mut self, value: V) -> &mut Self
    where
        V: Into<Direction>,
    {
        self.inner.direction(value);
        self
    }

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
    pub fn builder() -> GameDerivedGamesBuilder<'a> {
        GameDerivedGamesBuilder::default()
    }
}

impl<'a> GameRecords<'a> {
    pub fn builder() -> GameRecordsBuilder<'a> {
        GameRecordsBuilder::default()
    }
}

impl<'a> GameRecordsBuilder<'a> {
    pub fn embed(&mut self, embed: LeaderboardEmbeds) -> &mut Self {
        self.embed.get_or_insert_with(BTreeSet::new).insert(embed);
        self
    }

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
