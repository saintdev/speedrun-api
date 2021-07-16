use std::borrow::Cow;

use http::Method;
use serde::Serialize;

use crate::{
    api::{endpoint::Endpoint, error::BodyError},
    types::TimingMethod,
};

/// Embeds available for leaderboards.
///
/// NOTE: Embeds can be nested. That is not handled by this API.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LeaderboardEmbeds {
    Game,
    Category,
    Level,
    Players,
    Regions,
    Platforms,
    Variables,
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct FullGameLeaderboard<'a> {
    #[serde(skip)]
    game: Cow<'a, str>,
    #[serde(skip)]
    category: Cow<'a, str>,
    // NOTE: Make this a common struct?
    top: Option<i64>,
    platform: Option<Cow<'a, str>>,
    region: Option<Cow<'a, str>>,
    emulators: Option<bool>,
    video_only: Option<bool>,
    timing: Option<TimingMethod>,
    date: Option<String>,
    /*TODO
     * variables: HashMap<String, String>, */
    embed: Option<Vec<LeaderboardEmbeds>>,
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct IndividualLevelLeaderboard<'a> {
    #[serde(skip)]
    game: Cow<'a, str>,
    #[serde(skip)]
    level: Cow<'a, str>,
    #[serde(skip)]
    category: Cow<'a, str>,
    // NOTE: Make this a common struct?
    top: Option<i64>,
    platform: Option<Cow<'a, str>>,
    region: Option<Cow<'a, str>>,
    emulators: Option<bool>,
    video_only: Option<bool>,
    timing: Option<TimingMethod>,
    date: Option<String>,
    /*TODO
     * variables: HashMap<String, String>, */
    embed: Option<Vec<LeaderboardEmbeds>>,
}

impl<'a> FullGameLeaderboard<'a> {
    pub fn builder() -> FullGameLeaderboardBuilder<'a> {
        FullGameLeaderboardBuilder::default()
    }
}

impl<'a> IndividualLevelLeaderboard<'a> {
    pub fn builder() -> IndividualLevelLeaderboardBuilder<'a> {
        IndividualLevelLeaderboardBuilder::default()
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
