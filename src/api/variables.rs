//! # Variables
//!
//! Endpoints available for variables
use std::{borrow::Cow, fmt::Display};

use http::Method;
use serde::{Deserialize, Serialize};

use super::endpoint::Endpoint;

/// Represents a variable ID
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct VariableId<'a>(Cow<'a, str>);

impl<'a> VariableId<'a> {
    /// Create a new [`VariableId`]
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for VariableId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl Display for VariableId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// Represents a value ID
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct ValueId<'a>(Cow<'a, str>);

impl<'a> ValueId<'a> {
    /// Create a new [`ValueId`]
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for ValueId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl Display for ValueId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// Retrieves a single variable
#[derive(Debug, Builder, Clone)]
#[builder(setter(into, strip_option))]
pub struct Variable<'a> {
    #[doc = r"Variable ID"]
    id: VariableId<'a>,
}

impl<'a> Variable<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> VariableBuilder<'a> {
        VariableBuilder::default()
    }
}

impl Endpoint for Variable<'_> {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/variables/{}", self.id).into()
    }
}
