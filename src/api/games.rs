use std::borrow::Cow;

use http::Method;
use serde::Serialize;
use thiserror::Error;

use super::{
    endpoint::Endpoint, error::BodyError, CategoriesSorting, Direction, Pageable, VariablesSorting,
};

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
}

impl<'a> Games<'a> {
    pub fn builder() -> GamesBuilder<'a> {
        GamesBuilder::default()
    }
}

impl<'a> Endpoint for Games<'a> {
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

impl<'a> Pageable for Games<'a> {}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Game<'a> {
    id: Cow<'a, str>,
}

impl<'a> Game<'a> {
    pub fn builder() -> GameBuilder<'a> {
        GameBuilder::default()
    }
}

impl<'a> Endpoint for Game<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/games/{}", self.id).into()
    }
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

impl<'a> GameCategories<'a> {
    pub fn builder() -> GameCategoriesBuilder<'a> {
        GameCategoriesBuilder::default()
    }
}

impl<'a> Endpoint for GameCategories<'a> {
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

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct GameLevels<'a> {
    #[serde(skip)]
    id: Cow<'a, str>,
    orderby: Option<LevelsSorting>,
    direction: Option<Direction>,
}

/// Sorting options for levels
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum LevelsSorting {
    /// Sorts alphanumerically by the level name
    Name,
    /// Sorts by the order defined by the game moderator (default)
    Pos,
}

impl Default for LevelsSorting {
    fn default() -> Self {
        Self::Pos
    }
}

impl<'a> GameLevels<'a> {
    pub fn builder() -> GameLevelsBuilder<'a> {
        GameLevelsBuilder::default()
    }
}

impl<'a> Endpoint for GameLevels<'a> {
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

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct GameVariables<'a> {
    #[serde(skip)]
    id: Cow<'a, str>,
    orderby: Option<VariablesSorting>,
    direction: Option<Direction>,
}

impl<'a> GameVariables<'a> {
    pub fn builder() -> GameVariablesBuilder<'a> {
        GameVariablesBuilder::default()
    }
}

impl<'a> Endpoint for GameVariables<'a> {
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

#[derive(Default, Clone)]
pub struct GameDerivedGamesBuilder<'a> {
    id: Option<Cow<'a, str>>,
    inner: GamesBuilder<'a>,
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

#[derive(Debug, Error)]
pub enum GameDerivedGamesBuilderError {
    #[error("{0} must be initialized")]
    UninitializedField(&'static str),
    #[error(transparent)]
    Inner(#[from] GamesBuilderError),
}

#[derive(Default, Debug, Clone)]
pub struct GameDerivedGames<'a> {
    id: Cow<'a, str>,
    inner: Games<'a>,
}

impl<'a> GameDerivedGames<'a> {
    pub fn builder() -> GameDerivedGamesBuilder<'a> {
        GameDerivedGamesBuilder::default()
    }
}

impl<'a> Endpoint for GameDerivedGames<'a> {
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

impl<'a> Pageable for GameDerivedGames<'a> {}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct GameRecords<'a> {
    id: Cow<'a, str>,
    top: Option<i64>,
    scope: Option<LeaderboardScope>,
    miscellaneous: Option<bool>,
    skip_empty: Option<bool>,
}

impl<'a> GameRecords<'a> {
    pub fn builder() -> GameRecordsBuilder<'a> {
        GameRecordsBuilder::default()
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum LeaderboardScope {
    FullGame,
    Levels,
    All,
}

impl<'a> Endpoint for GameRecords<'a> {
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

impl Pageable for GameRecords<'_> {}
