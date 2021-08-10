use serde::Deserialize;

use crate::api::genres::GenreId;

use super::Link;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Genre<'a> {
    pub id: GenreId<'a>,
    pub name: String,
    pub links: Vec<Link>,
}
