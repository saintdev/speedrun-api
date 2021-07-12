use serde::Deserialize;

use super::Link;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GameType {
    pub id: String,
    pub name: String,
    pub allows_base_game: bool,
    pub links: Vec<Link>,
}
