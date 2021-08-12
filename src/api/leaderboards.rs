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
    Game,
    Category,
    Level,
    Players,
    Regions,
    Platforms,
    Variables,
}

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct FullGameLeaderboard<'a> {
    #[serde(skip)]
    game: GameId<'a>,
    #[serde(skip)]
    category: CategoryId<'a>,
    // NOTE: Make this a common struct?
    #[builder(default)]
    top: Option<i64>,
    #[builder(default)]
    platform: Option<PlatformId<'a>>,
    #[builder(default)]
    region: Option<RegionId<'a>>,
    #[builder(default)]
    emulators: Option<bool>,
    #[builder(default)]
    video_only: Option<bool>,
    #[builder(default)]
    timing: Option<TimingMethod>,
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

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct IndividualLevelLeaderboard<'a> {
    #[serde(skip)]
    game: GameId<'a>,
    #[serde(skip)]
    level: LevelId<'a>,
    #[serde(skip)]
    category: CategoryId<'a>,
    // NOTE: Make this a common struct?
    #[builder(default)]
    top: Option<i64>,
    #[builder(default)]
    platform: Option<PlatformId<'a>>,
    #[builder(default)]
    region: Option<RegionId<'a>>,
    #[builder(default)]
    emulators: Option<bool>,
    #[builder(default)]
    video_only: Option<bool>,
    #[builder(default)]
    timing: Option<TimingMethod>,
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
    pub fn builder() -> FullGameLeaderboardBuilder<'a> {
        FullGameLeaderboardBuilder::default()
    }
}

impl<'a> FullGameLeaderboardBuilder<'a> {
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
    pub fn builder() -> IndividualLevelLeaderboardBuilder<'a> {
        IndividualLevelLeaderboardBuilder::default()
    }
}

impl<'a> IndividualLevelLeaderboardBuilder<'a> {
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
                .map(|(var, val)| format!("var-{}={}", var, val)),
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
                .map(|(var, val)| format!("var-{}={}", var, val)),
        );

        Ok(params.join("&").into())
    }
}

impl From<&LeaderboardEmbeds> for &'static str {
    fn from(value: &LeaderboardEmbeds) -> Self {
        value.as_str()
    }
}
