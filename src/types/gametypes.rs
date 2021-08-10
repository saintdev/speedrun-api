use serde::Deserialize;

use crate::api::gametypes::GameTypeId;

use super::Link;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GameType<'a> {
    pub id: GameTypeId<'a>,
    pub name: String,
    pub allows_base_game: bool,
    pub links: Vec<Link>,
}
