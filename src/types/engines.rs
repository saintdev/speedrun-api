use serde::Deserialize;

use crate::api::engines::EngineId;

use super::Link;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Engine<'a> {
    pub id: EngineId<'a>,
    pub name: String,
    pub links: Vec<Link>,
}
