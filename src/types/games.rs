use std::collections::HashMap;

use serde::Deserialize;

use super::{Link, TimingMethod};

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
    pub twitch: String,
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Assets {
    pub logo: Asset,
    pub cover_tiny: Asset,
    pub cover_small: Asset,
    pub cover_medium: Asset,
    pub cover_large: Asset,
    pub icon: Asset,
    pub trophy_1st: Asset,
    pub trophy_2nd: Asset,
    pub trophy_3rd: Asset,
    pub trophy_4th: Option<Asset>,
    pub background: Option<Asset>,
    pub foreground: Option<Asset>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Asset {
    pub uri: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModeratorRole {
    Moderator,
    SuperModerator,
}
