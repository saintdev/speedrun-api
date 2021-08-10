use std::collections::HashMap;

use serde::Deserialize;

use crate::api::{
    categories::CategoryId, games::GameId, levels::LevelId, platforms::PlatformId,
    regions::RegionId, variables::VariableId,
};

use super::{common::TimingMethod, Link, Run};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Leaderboard<'a> {
    pub weblink: String,
    pub game: GameId<'a>,
    pub category: CategoryId<'a>,
    #[serde(default)]
    pub level: Option<LevelId<'a>>,
    #[serde(default)]
    pub platform: Option<PlatformId<'a>>,
    #[serde(default)]
    pub region: Option<RegionId<'a>>,
    #[serde(default)]
    pub emulators: Option<bool>,
    pub video_only: bool,
    #[serde(default)]
    pub timing: Option<TimingMethod>,
    #[serde(default)]
    pub values: HashMap<VariableId<'a>, String>,
    pub runs: Vec<RankedRun<'a>>,
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RankedRun<'a> {
    pub place: i64,
    pub run: Run<'a>,
}
