use std::fmt::Display;

use crate::api::engines::EngineId;

use super::Engine;

impl<'a> From<Engine<'a>> for EngineId<'a> {
    fn from(value: Engine<'a>) -> Self {
        value.id
    }
}

impl Display for Engine<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}
