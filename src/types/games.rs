use std::collections::HashMap;

use serde::Deserialize;

use crate::api::{
    developers::DeveloperId, engines::EngineId, games::GameId, gametypes::GameTypeId,
    genres::GenreId, platforms::PlatformId, publishers::PublisherId, regions::RegionId,
    users::UserId,
};

use super::{Assets, Link, ModeratorRole, Names, TimingMethod};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Game<'a> {
    pub id: GameId<'a>,
    pub names: Names,
    pub abbreviation: String,
    pub weblink: String,
    pub release_date: String,
    pub ruleset: Ruleset,
    pub gametypes: Vec<GameTypeId<'a>>,
    pub platforms: Vec<PlatformId<'a>>,
    pub regions: Vec<RegionId<'a>>,
    pub genres: Vec<GenreId<'a>>,
    pub engines: Vec<EngineId<'a>>,
    pub developers: Vec<DeveloperId<'a>>,
    pub publishers: Vec<PublisherId<'a>>,
    pub moderators: HashMap<UserId<'a>, ModeratorRole>,
    pub created: Option<String>,
    pub assets: Assets,
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Ruleset {
    pub show_milliseconds: bool,
    pub require_verification: bool,
    pub require_video: bool,
    pub run_times: Vec<TimingMethod>,
    pub default_time: TimingMethod,
    pub emulators_allowed: bool,
}
