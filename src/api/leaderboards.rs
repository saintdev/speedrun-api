use std::borrow::Cow;

use http::Method;
use serde::Serialize;

use crate::{
    api::{endpoint::Endpoint, error::BodyError},
    types::TimingMethod,
};

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
}

impl<'a> FullGameLeaderboard<'a> {
    pub fn builder() -> FullGameLeaderboardBuilder<'a> {
        FullGameLeaderboardBuilder::default()
    }
}

impl<'a> Endpoint for FullGameLeaderboard<'a> {
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
}

impl<'a> IndividualLevelLeaderboard<'a> {
    pub fn builder() -> IndividualLevelLeaderboardBuilder<'a> {
        IndividualLevelLeaderboardBuilder::default()
    }
}

impl<'a> Endpoint for IndividualLevelLeaderboard<'a> {
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
