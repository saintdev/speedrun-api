use std::collections::HashMap;

use serde::Deserialize;

use crate::api::{
    categories::CategoryId, games::GameId, levels::LevelId, platforms::PlatformId,
    regions::RegionId, runs::RunId, users::UserId, variables::VariableId,
};

use super::Link;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Run<'a> {
    pub id: RunId<'a>,
    pub weblink: String,
    pub game: GameId<'a>,
    #[serde(default)]
    pub level: Option<LevelId<'a>>,
    pub category: CategoryId<'a>,
    #[serde(default)]
    pub videos: Option<Videos>,
    #[serde(default)]
    pub comment: Option<String>,
    pub status: Status<'a>,
    pub players: Vec<Player<'a>>,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub submitted: Option<String>,
    pub times: Times,
    pub system: System<'a>,
    #[serde(default)]
    pub splits: Option<Link>,
    #[serde(default)]
    pub values: HashMap<VariableId<'a>, String>,
    #[serde(default)]
    pub links: Option<Vec<Link>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Videos {
    pub text: Option<String>,
    #[serde(default)]
    pub links: Vec<VideoLink>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct VideoLink {
    pub uri: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "status")]
pub enum Status<'a> {
    New,
    #[serde(rename_all = "kebab-case")]
    Verified {
        examiner: Option<UserId<'a>>,
        verify_date: Option<String>,
    },
    Rejected {
        examiner: UserId<'a>,
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "rel")]
pub enum Player<'a> {
    User { id: UserId<'a>, uri: String },
    Guest { name: String, uri: String },
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Times {
    pub primary: String,
    pub primary_t: f64,
    pub realtime: Option<String>,
    pub realtime_t: f64,
    pub realtime_noloads: Option<String>,
    pub realtime_noloads_t: f64,
    pub ingame: Option<String>,
    pub ingame_t: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct System<'a> {
    pub platform: Option<PlatformId<'a>>,
    pub emulated: bool,
    pub region: Option<RegionId<'a>>,
}
