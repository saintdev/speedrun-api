use std::{borrow::Cow, collections::BTreeSet};

use http::Method;
use serde::Serialize;

use crate::{
    api::{endpoint::Endpoint, error::BodyError},
    types::TimingMethod,
};

use super::{
    categories::CategoryId, games::GameId, levels::LevelId, platforms::PlatformId,
    regions::RegionId,
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
    /*TODO
     * variables: HashMap<String, String>, */
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
    /*TODO
     * variables: HashMap<String, String>, */
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
        Ok(serde_urlencoded::to_string(self)?.into())
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
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl From<&LeaderboardEmbeds> for &'static str {
    fn from(value: &LeaderboardEmbeds) -> Self {
        value.as_str()
    }
}
