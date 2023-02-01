//! # Leaderboards
//!
//! Endpoints available for leaderboards

use std::{
    borrow::Cow,
    collections::{BTreeSet, HashMap},
};

use http::Method;
use serde::Serialize;

use crate::{
    api::{endpoint::Endpoint, error::BodyError},
    types::TimingMethod,
};

use super::{
    categories::CategoryId,
    games::GameId,
    levels::LevelId,
    platforms::PlatformId,
    regions::RegionId,
    variables::{ValueId, VariableId},
};

/// Embeds available for leaderboards.
///
/// NOTE: Embeds can be nested. That is not handled by this API.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LeaderboardEmbeds {
    /// Embed the full game resource.
    Game,
    /// Embed the category used for the leaderboard.
    Category,
    /// Embed the level used for the leaderboard.
    Level,
    /// Adds a new `players` element to the leaderboard, containing a flat list
    /// of all players of all runs on the leaderboard.
    Players,
    /// Adds all used regions.
    Regions,
    /// Adds all used platforms.
    Platforms,
    /// Adds all applicable variables for the chosen level/categories
    Variables,
}

/// Retrieves a full-game leaderboard identified by game and category.
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct FullGameLeaderboard<'a> {
    #[doc = r"Game `ID` or abbreviation."]
    #[serde(skip)]
    game: GameId<'a>,
    #[doc = r"Category `ID` or abbreviation."]
    #[serde(skip)]
    category: CategoryId<'a>,

    // NOTE: Make the below fields a common struct for full-games and individual-levels?
    #[doc = r"Only return `top` places."]
    #[builder(default)]
    top: Option<i64>,
    #[doc = r"Only return runs done on `platform`."]
    #[builder(default)]
    platform: Option<PlatformId<'a>>,
    #[doc = r"Only return runs done in `region`."]
    #[builder(default)]
    region: Option<RegionId<'a>>,
    #[doc = r"When unset, real devices and emulator results are returned. When `true` only emulator runs are returned, otherwise only real deivces are returned."]
    #[builder(default)]
    emulators: Option<bool>,
    #[doc = r"When `true` only runs with videos will be returned. (default: `false`)"]
    #[builder(default)]
    video_only: Option<bool>,
    #[doc = r"What [`TimingMethod`] to use to determine the sorting of runs."]
    #[builder(default)]
    timing: Option<TimingMethod>,
    #[doc = r"Only return runs done on or before this date. [ISO 8601 date string](https://en.wikipedia.org/wiki/ISO_8601#Dates)."]
    #[builder(default)]
    date: Option<String>,
    #[builder(setter(name = "_variables"), private, default)]
    #[serde(skip)]
    variables: HashMap<VariableId<'a>, ValueId<'a>>,
    #[builder(setter(name = "_embed"), private, default)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<LeaderboardEmbeds>,
}

/// Retrieves an individual-level leaderboard identified by game, category and
/// level.
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct IndividualLevelLeaderboard<'a> {
    #[doc = r"Game `ID` or abbreviation."]
    #[serde(skip)]
    game: GameId<'a>,
    #[doc = r"Level `ID` or abbreviation."]
    #[serde(skip)]
    level: LevelId<'a>,
    #[doc = r"Category `ID` or abbreviation."]
    #[serde(skip)]
    category: CategoryId<'a>,

    // NOTE: Make the below fields a common struct for full-games and individual-levels?
    #[doc = r"Only return `top` places."]
    #[builder(default)]
    top: Option<i64>,
    #[doc = r"Only return runs done on `platform`."]
    #[builder(default)]
    platform: Option<PlatformId<'a>>,
    #[doc = r"Only return runs done in `region`."]
    #[builder(default)]
    region: Option<RegionId<'a>>,
    #[doc = r"When unset, real devices and emulator results are returned. When `true` only emulator runs are returned, otherwise only real deivces are returned."]
    #[builder(default)]
    emulators: Option<bool>,
    #[doc = r"When `true` only runs with videos will be returned. (default: `false`)"]
    #[builder(default)]
    video_only: Option<bool>,
    #[doc = r"What [`TimingMethod`] to use to determine the sorting of runs."]
    #[builder(default)]
    timing: Option<TimingMethod>,
    #[doc = r"Only return runs done on or before this date. [ISO 8601 date string](https://en.wikipedia.org/wiki/ISO_8601#Dates)."]
    #[builder(default)]
    date: Option<String>,
    #[builder(setter(name = "_variables"), private, default)]
    #[serde(skip)]
    variables: HashMap<VariableId<'a>, ValueId<'a>>,
    #[builder(setter(name = "_embed"), private, default)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<LeaderboardEmbeds>,
}

impl<'a> FullGameLeaderboard<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> FullGameLeaderboardBuilder<'a> {
        FullGameLeaderboardBuilder::default()
    }
}

impl<'a> FullGameLeaderboardBuilder<'a> {
    /// Add an embedded resource to this result
    pub fn embed(&mut self, embed: LeaderboardEmbeds) -> &mut Self {
        self.embed.get_or_insert_with(BTreeSet::new).insert(embed);
        self
    }

    /// Add multiple embedded resources to this result
    pub fn embeds<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = LeaderboardEmbeds>,
    {
        self.embed.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }

    /// Add a single custom variable to filter results by.
    pub fn variable<Var, Val>(&mut self, variable: Var, value: Val) -> &mut Self
    where
        Var: Into<VariableId<'a>>,
        Val: Into<ValueId<'a>>,
    {
        self.variables
            .get_or_insert_with(HashMap::new)
            .insert(variable.into(), value.into());
        self
    }

    /// Add multiple custom variables to filter results by.
    pub fn variables<I, Var, Val>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = (Var, Val)>,
        Var: Into<VariableId<'a>>,
        Val: Into<ValueId<'a>>,
    {
        self.variables
            .get_or_insert_with(HashMap::new)
            .extend(iter.into_iter().map(|(k, v)| (k.into(), v.into())));
        self
    }
}

impl<'a> IndividualLevelLeaderboard<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> IndividualLevelLeaderboardBuilder<'a> {
        IndividualLevelLeaderboardBuilder::default()
    }
}

impl<'a> IndividualLevelLeaderboardBuilder<'a> {
    /// Add an embedded resource to this result
    pub fn embed(&mut self, embed: LeaderboardEmbeds) -> &mut Self {
        self.embed.get_or_insert_with(BTreeSet::new).insert(embed);
        self
    }

    /// Add multiple embedded resources to this result
    pub fn embeds<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = LeaderboardEmbeds>,
    {
        self.embed.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }

    /// Add a single custom variable to filter results by.
    pub fn variable<Var, Val>(&mut self, variable: Var, value: Val) -> &mut Self
    where
        Var: Into<VariableId<'a>>,
        Val: Into<ValueId<'a>>,
    {
        self.variables
            .get_or_insert_with(HashMap::new)
            .insert(variable.into(), value.into());
        self
    }

    /// Add multiple custom variables to filter results by.
    pub fn variables<I, Var, Val>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = (Var, Val)>,
        Var: Into<VariableId<'a>>,
        Val: Into<ValueId<'a>>,
    {
        self.variables
            .get_or_insert_with(HashMap::new)
            .extend(iter.into_iter().map(|(k, v)| (k.into(), v.into())));
        self
    }
}

impl LeaderboardEmbeds {
    fn as_str(&self) -> &'static str {
        match self {
            LeaderboardEmbeds::Game => "game",
            LeaderboardEmbeds::Category => "category",
            LeaderboardEmbeds::Level => "level",
            LeaderboardEmbeds::Players => "players",
            LeaderboardEmbeds::Regions => "regions",
            LeaderboardEmbeds::Platforms => "platforms",
            LeaderboardEmbeds::Variables => "variables",
        }
    }
}

impl Endpoint for FullGameLeaderboard<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/leaderboards/{}/category/{}", self.game, self.category).into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, BodyError> {
        let mut params = Vec::new();
        let urlencoded = serde_urlencoded::to_string(self)?;
        if !urlencoded.is_empty() {
            params.push(urlencoded);
        }

        params.extend(
            self.variables
                .iter()
                .map(|(var, val)| format!("var-{var}={val}")),
        );

        Ok(params.join("&").into())
    }
}

impl Endpoint for IndividualLevelLeaderboard<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "/leaderboards/{}/level/{}/{}",
            self.game, self.level, self.category
        )
        .into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, BodyError> {
        let mut params = Vec::new();
        let urlencoded = serde_urlencoded::to_string(self)?;
        if !urlencoded.is_empty() {
            params.push(urlencoded);
        }

        params.extend(
            self.variables
                .iter()
                .map(|(var, val)| format!("var-{var}={val}")),
        );

        Ok(params.join("&").into())
    }
}

impl From<&LeaderboardEmbeds> for &'static str {
    fn from(value: &LeaderboardEmbeds) -> Self {
        value.as_str()
    }
}
