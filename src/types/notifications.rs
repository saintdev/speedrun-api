use serde::Deserialize;

use super::Link;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Notification {
    pub id: String,
    pub created: String,
    pub status: ReadStatus,
    pub text: String,
    pub item: Item,
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "rel", content = "uri")]
pub enum Item {
    Post(String),
    Run(String),
    Game(String),
    Guide(String),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReadStatus {
    Read,
    Unread,
}
