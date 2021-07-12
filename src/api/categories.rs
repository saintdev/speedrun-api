use std::borrow::Cow;

use http::Method;
use serde::Serialize;

use super::{endpoint::Endpoint, error::BodyError, Direction, Pageable, VariablesSorting};

/// Retrieves a single category, identified by it's ID
#[derive(Debug, Builder)]
pub struct Category<'a> {
    #[builder(setter(into))]
    #[doc = r"`id` of this category."]
    id: Cow<'a, str>,
}

/// Retrieves all variables that are applicable to the category identified by
/// ID.
#[derive(Debug, Builder, Serialize)]
pub struct CategoryVariables<'a> {
    #[builder(setter(into))]
    #[serde(skip)]
    #[doc = r"`id` of the category to retrieve variables for."]
    id: Cow<'a, str>,
    #[builder(default, setter(strip_option))]
    #[doc = r"Sorting for results"]
    orderby: Option<VariablesSorting>,
    #[builder(default, setter(strip_option))]
    #[doc = r"Sort direction"]
    direction: Option<Direction>,
}

/// Retrieves the records for the given category id.
#[derive(Debug, Builder, Serialize)]
pub struct CategoryRecords<'a> {
    #[builder(setter(into))]
    #[serde(skip)]
    #[doc = r"`id` for the category."]
    id: Cow<'a, str>,
    #[builder(default)]
    #[doc = r"Return `top` number of places (default: 3)."]
    top: Option<u32>,
    #[builder(default)]
    #[doc = r"Do not return empty leaderboards when true"]
    skip_empty: Option<bool>,
}

impl<'a> Category<'a> {
    /// Create a builder for this endpoint
    pub fn builder() -> CategoryBuilder<'a> {
        CategoryBuilder::default()
    }
}

impl<'a> CategoryVariables<'a> {
    /// Create a builder for this endpoint
    pub fn builder() -> CategoryVariablesBuilder<'a> {
        CategoryVariablesBuilder::default()
    }
}

impl<'a> CategoryRecords<'a> {
    /// Create a builder for this endpoint
    pub fn builder() -> CategoryRecordsBuilder<'a> {
        CategoryRecordsBuilder::default()
    }
}

impl<'a> Endpoint for Category<'a> {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/categories/{}", self.id).into()
    }
}

impl<'a> Endpoint for CategoryVariables<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/categories/{}/variables", self.id).into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl<'a> Endpoint for CategoryRecords<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/categories/{}/records", self.id).into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Pageable for CategoryRecords<'_> {}
