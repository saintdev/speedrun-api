use crate::api::engines::EngineId;

use super::Engine;

impl<'a> From<Engine<'a>> for EngineId<'a> {
    fn from(value: Engine<'a>) -> Self {
        value.id
    }
}
