use std::borrow::Cow;

use http::Method;
use serde::Serialize;

use super::{endpoint::Endpoint, Direction, Pageable};

/// Sorting options for engines
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum EnginesSorting {
    /// Sort alphanumerically by engine name (default)
    Name,
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Engines {
    orderby: Option<EnginesSorting>,
    direction: Option<Direction>,
}

#[derive(Default, Debug, Builder, Clone)]
#[builder(default, setter(into, strip_option))]
pub struct Engine<'a> {
    id: Cow<'a, str>,
}

impl Engines {
    pub fn builder() -> EnginesBuilder {
        EnginesBuilder::default()
    }
}

impl<'a> Engine<'a> {
    pub fn builder() -> EngineBuilder<'a> {
        EngineBuilder::default()
    }
}

impl Default for EnginesSorting {
    fn default() -> Self {
        Self::Name
    }
}

impl Endpoint for Engines {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "/engines".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Endpoint for Engine<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/engines/{}", self.id).into()
    }
}

impl Pageable for Engines {}
