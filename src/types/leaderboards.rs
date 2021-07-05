use std::collections::HashMap;

use serde::Deserialize;

use super::{common::TimingMethod, Link, Run};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Leaderboard {
    pub weblink: String,
    pub game: String,
    pub category: String,
    pub level: Option<String>,
    pub platform: Option<String>,
    pub region: Option<String>,
    pub emulators: Option<bool>,
    pub video_only: bool,
    pub timing: Option<TimingMethod>,
    pub values: HashMap<String, String>,
    pub runs: Vec<RankedRun>,
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RankedRun {
    pub place: i64,
    pub run: Run,
}
