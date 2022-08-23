//! # Categories
//!
//! Endpoints available for categories

use std::{borrow::Cow, collections::BTreeSet, fmt::Display};

use http::Method;
use serde::{Deserialize, Serialize};

use super::{
    endpoint::Endpoint, error::BodyError, leaderboards::LeaderboardEmbeds, Direction, Pageable,
    VariablesSorting,
};

/// Embeds available for categories.
///
/// NOTE: Embeds can be nested. That is not handled by this API.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CategoryEmbeds {
    /// Embed the `game` resource this category belongs to.
    Game,
    /// Embed `variables` applicable to this category
    Variables,
}

/// Represents a category ID
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct CategoryId<'a>(Cow<'a, str>);

impl<'a> CategoryId<'a> {
    /// Create a new [`CategoryId`]
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for CategoryId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        CategoryId::new(value)
    }
}

impl Display for CategoryId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// Retrieves a single category, identified by it's ID
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
pub struct Category<'a> {
    #[serde(skip)]
    #[doc = r"`ID` of this category."]
    id: CategoryId<'a>,
    #[builder(setter(name = "_embed"), private, default)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<CategoryEmbeds>,
}

/// Retrieves all variables that are applicable to the category identified by
/// ID.
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct CategoryVariables<'a> {
    #[serde(skip)]
    #[doc = r"`ID` of the category to retrieve variables for."]
    id: CategoryId<'a>,
    #[doc = r"Sorting options for results."]
    #[builder(default)]
    orderby: Option<VariablesSorting>,
    #[doc = r"Sort direction"]
    #[builder(default)]
    direction: Option<Direction>,
}

/// Retrieves the records for the given category id.
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct CategoryRecords<'a> {
    #[serde(skip)]
    #[doc = r"`ID` for the category."]
    id: CategoryId<'a>,
    #[doc = r"Return `top` number of places (default: 3)."]
    #[builder(default)]
    top: Option<u32>,
    #[doc = r"Do not return empty leaderboards when `true`."]
    #[builder(default)]
    skip_empty: Option<bool>,
    #[builder(setter(name = "_embed"), private, default)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<LeaderboardEmbeds>,
}

impl<'a> Category<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> CategoryBuilder<'a> {
        CategoryBuilder::default()
    }
}

impl<'a> CategoryBuilder<'a> {
    /// Add an embedded resource to this result
    pub fn embed(&mut self, embed: CategoryEmbeds) -> &mut Self {
        self.embed.get_or_insert_with(BTreeSet::new).insert(embed);
        self
    }

    /// Add multiple embedded resources to this result
    pub fn embeds<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = CategoryEmbeds>,
    {
        self.embed.get_or_insert_with(BTreeSet::new).extend(iter);
        self
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

impl<'a> CategoryRecordsBuilder<'a> {
    /// Add an embedded resource to this result
    pub fn embed(&mut self, embed: LeaderboardEmbeds) -> &mut Self {
        self.embed.get_or_insert_with(BTreeSet::new).insert(embed);
        self
    }

    /// Add multiple embedded resources to this result
    pub fn embeds<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = LeaderboardEmbeds>,
    {
        self.embed.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }
}

impl CategoryEmbeds {
    fn as_str(&self) -> &'static str {
        match self {
            CategoryEmbeds::Game => "game",
            CategoryEmbeds::Variables => "variables",
        }
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

impl From<&CategoryEmbeds> for &'static str {
    fn from(value: &CategoryEmbeds) -> Self {
        value.as_str()
    }
}

impl Pageable for CategoryRecords<'_> {}
