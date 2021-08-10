use serde::Deserialize;

use crate::api::categories::CategoryId;

use super::Link;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Category<'a> {
    pub id: CategoryId<'a>,
    pub name: String,
    pub weblink: String,
    #[serde(rename = "type")]
    pub type_field: CategoryType,
    pub rules: String,
    pub players: Players,
    pub miscellaneous: bool,
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CategoryType {
    PerGame,
    PerLevel,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type", content = "value")]
pub enum Players {
    Exactly(i64),
    UpTo(i64),
}
