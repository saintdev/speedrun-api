use std::borrow::Cow;

use http::Method;
use serde::Serialize;

use super::{
    endpoint::Endpoint, error::BodyError, leaderboards::LeaderboardEmbeds, Direction, Pageable,
    VariablesSorting,
};

/// Embeds available for categories.
///
/// NOTE: Embeds can be nested. That is not handled by this API.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CategoryEmbeds {
    Game,
    Variables,
}

/// Retrieves a single category, identified by it's ID
#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
pub struct Category<'a> {
    #[serde(skip)]
    #[doc = r"`id` of this category."]
    id: Cow<'a, str>,
    #[doc = r"Resources to embed in the result"]
    embed: Option<Vec<CategoryEmbeds>>,
}

/// Retrieves all variables that are applicable to the category identified by
/// ID.
#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct CategoryVariables<'a> {
    #[serde(skip)]
    #[doc = r"`id` of the category to retrieve variables for."]
    id: Cow<'a, str>,
    #[doc = r"Sorting for results"]
    orderby: Option<VariablesSorting>,
    #[doc = r"Sort direction"]
    direction: Option<Direction>,
}

/// Retrieves the records for the given category id.
#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct CategoryRecords<'a> {
    #[serde(skip)]
    #[doc = r"`id` for the category."]
    id: Cow<'a, str>,
    #[doc = r"Return `top` number of places (default: 3)."]
    top: Option<u32>,
    #[doc = r"Do not return empty leaderboards when true"]
    skip_empty: Option<bool>,
    embed: Vec<LeaderboardEmbeds>,
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

impl Endpoint for Category<'_> {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/categories/{}", self.id).into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Endpoint for CategoryVariables<'_> {
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

impl Endpoint for CategoryRecords<'_> {
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
