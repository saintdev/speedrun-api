use std::collections::HashMap;

use serde::Deserialize;

use super::Link;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Run {
    pub id: String,
    pub weblink: String,
    pub game: String,
    pub level: Option<String>,
    pub category: String,
    pub videos: Option<Videos>,
    pub comment: Option<String>,
    pub status: Status,
    pub players: Vec<Player>,
    pub date: Option<String>,
    pub submitted: Option<String>,
    pub times: Times,
    pub system: System,
    pub splits: Option<Link>,
    pub values: HashMap<String, String>,
    pub links: Option<Vec<Link>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Videos {
    pub text: Option<String>,
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
pub enum Status {
    New,
    Verified {
        examiner: Option<String>,
        verify_date: Option<String>,
    },
    Rejected {
        examiner: String,
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "rel")]
pub enum Player {
    User { id: String, uri: String },
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
pub struct System {
    pub platform: Option<String>,
    pub emulated: bool,
    pub region: Option<String>,
}
