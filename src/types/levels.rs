use crate::{api::levels::LevelId, types::Link};
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Level<'a> {
    pub id: LevelId<'a>,
    pub name: String,
    pub weblink: String,
    #[serde(default)]
    pub rules: Option<String>,
    pub links: Vec<Link>,
}
