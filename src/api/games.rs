use std::{borrow::Cow, collections::BTreeSet};

use http::Method;
use serde::Serialize;
use thiserror::Error;

use super::{
    endpoint::Endpoint, error::BodyError, leaderboards::LeaderboardEmbeds, CategoriesSorting,
    Direction, Pageable, VariablesSorting,
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

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Games<'a> {
    name: Option<Cow<'a, str>>,
    abbreviation: Option<Cow<'a, str>>,
    released: Option<i64>,
    gametype: Option<Cow<'a, str>>,
    platform: Option<Cow<'a, str>>,
    region: Option<Cow<'a, str>>,
    genre: Option<Cow<'a, str>>,
    engine: Option<Cow<'a, str>>,
    developer: Option<Cow<'a, str>>,
    publisher: Option<Cow<'a, str>>,
    moderator: Option<Cow<'a, str>>,
    #[serde(rename = "_bulk")]
    bulk: Option<bool>,
    orderby: Option<GamesSorting>,
    direction: Option<Direction>,
    #[builder(setter(name = "_embed"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<GameEmbeds>,
}

#[derive(Default, Debug, Builder, Clone)]
#[builder(setter(into, strip_option))]
pub struct Game<'a> {
    id: Cow<'a, str>,
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct GameCategories<'a> {
    #[serde(skip)]
    id: Cow<'a, str>,
    miscellaneous: Option<bool>,
    orderby: Option<CategoriesSorting>,
    direction: Option<Direction>,
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct GameLevels<'a> {
    #[serde(skip)]
    id: Cow<'a, str>,
    orderby: Option<LevelsSorting>,
    direction: Option<Direction>,
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct GameVariables<'a> {
    #[serde(skip)]
    id: Cow<'a, str>,
    orderby: Option<VariablesSorting>,
    direction: Option<Direction>,
}

#[derive(Default, Clone)]
pub struct GameDerivedGamesBuilder<'a> {
    id: Option<Cow<'a, str>>,
    inner: GamesBuilder<'a>,
}

#[derive(Default, Debug, Clone)]
pub struct GameDerivedGames<'a> {
    id: Cow<'a, str>,
    inner: Games<'a>,
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct GameRecords<'a> {
    id: Cow<'a, str>,
    top: Option<i64>,
    scope: Option<LeaderboardScope>,
    miscellaneous: Option<bool>,
    skip_empty: Option<bool>,
    #[builder(setter(name = "_embed"), private)]
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
        S: Into<Cow<'a, str>>,
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
