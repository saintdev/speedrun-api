use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Link {
    pub rel: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Pagination {
    pub offset: usize,
    pub max: usize,
    pub size: usize,
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TimingMethod {
    Realtime,
    RealtimeNoloads,
    Ingame,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModeratorRole {
    Moderator,
    SuperModerator,
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
    pub uri: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Names {
    pub international: String,
    pub japanese: Option<String>,
    pub twitch: Option<String>,
}
