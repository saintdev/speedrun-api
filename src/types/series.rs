use std::collections::HashMap;

use serde::Deserialize;

use crate::api::{series::SeriesId, users::UserId};

use super::{Assets, Link, ModeratorRole, Names};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Series<'a> {
    pub id: SeriesId<'a>,
    pub names: Names,
    pub abbreviation: String,
    pub weblink: String,
    #[serde(default)]
    pub moderators: HashMap<UserId<'a>, ModeratorRole>,
    #[serde(default)]
    pub created: Option<String>,
    pub assets: Assets,
    pub links: Vec<Link>,
}
