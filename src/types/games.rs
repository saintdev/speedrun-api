use std::collections::HashMap;

use serde::Deserialize;

use super::{Assets, Link, ModeratorRole, TimingMethod};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Game {
    pub id: String,
    pub names: Names,
    pub abbreviation: String,
    pub weblink: String,
    pub release_date: String,
    pub ruleset: Ruleset,
    pub gametypes: Vec<String>,
    pub platforms: Vec<String>,
    pub regions: Vec<String>,
    pub genres: Vec<String>,
    pub engines: Vec<String>,
    pub developers: Vec<String>,
    pub publishers: Vec<String>,
    pub moderators: HashMap<String, ModeratorRole>,
    pub created: Option<String>,
    pub assets: Assets,
    pub links: Vec<Link>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Names {
    pub international: String,
    pub japanese: Option<String>,
    pub twitch: Option<String>,
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
