//! # Engines
//!
//! Endpoints relating to engines.

use std::{borrow::Cow, fmt::Display};

use serde::{Deserialize, Serialize};

use super::{endpoint::Endpoint, error::BodyError, query_params::QueryParams, Direction, Pageable};

/// Sorting options for engines
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum EnginesSorting {
    /// Sort alphanumerically by engine name (default)
    Name,
}

/// Represents an engine ID.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct EngineId<'a>(Cow<'a, str>);

impl<'a> EngineId<'a> {
    /// Create a new [`EngineId`]
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for EngineId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl Display for EngineId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// Retrieves a list of engines.
#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Engines {
    #[doc = r"Sorting options for engines"]
    orderby: Option<EnginesSorting>,
    #[doc = r"Sort direction"]
    direction: Option<Direction>,
}

/// Retrieves a single engine represented by `ID`.
#[derive(Debug, Builder, Clone)]
#[builder(setter(into, strip_option))]
pub struct Engine<'a> {
    #[doc = r"`ID` of the engine"]
    id: EngineId<'a>,
}

impl Engines {
    /// Create a builder for this endpoint.
    pub fn builder() -> EnginesBuilder {
        EnginesBuilder::default()
    }
}

impl Engine<'_> {
    /// Create a builder for this endpoint.
    pub fn builder<'a>() -> EngineBuilder<'a> {
        EngineBuilder::default()
    }
}

impl Default for EnginesSorting {
    fn default() -> Self {
        Self::Name
    }
}

impl Endpoint for Engines {
    fn endpoint(&self) -> Cow<'static, str> {
        "/engines".into()
    }

    fn query_parameters(&self) -> Result<QueryParams<'_>, BodyError> {
        QueryParams::with(self)
    }
}

impl Endpoint for Engine<'_> {
    fn endpoint(&self) -> Cow<'static, str> {
        format!("/engines/{}", self.id).into()
    }
}

impl Pageable for Engines {}
