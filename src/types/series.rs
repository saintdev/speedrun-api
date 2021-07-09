use std::collections::HashMap;

use serde::Deserialize;

use super::{Assets, Link, ModeratorRole, Names};

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Series {
    pub id: String,
    pub names: Names,
    pub abbreviation: String,
    pub weblink: String,
    pub moderators: HashMap<String, ModeratorRole>,
    pub created: Option<String>,
    pub assets: Assets,
    pub links: Vec<Link>,
}
