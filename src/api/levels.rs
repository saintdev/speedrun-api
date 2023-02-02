//! # Levels
//!
//! Endpoints available for levels.
use std::{borrow::Cow, collections::BTreeSet, fmt::Display};

use http::Method;
use serde::{Deserialize, Serialize};

use super::{
    endpoint::Endpoint, error::BodyError, leaderboards::LeaderboardEmbeds,
    query_params::QueryParams, CategoriesSorting, Direction, Pageable, VariablesSorting,
};

/// Embeds available for levels.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LevelEmbeds {
    /// Embed per-level categories applicable to the requested level.
    Categories,
    /// Embed the variables applicable to the requested level.
    Variables,
}

/// Represents a level ID.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct LevelId<'a>(Cow<'a, str>);

impl<'a> LevelId<'a> {
    /// Create a new [`LevelId`].
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for LevelId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        LevelId::new(value)
    }
}

impl Display for LevelId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// Retrieve a single level, itentified by its ID.
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
pub struct Level<'a> {
    #[doc = r"`ID` of the level."]
    #[serde(skip)]
    id: LevelId<'a>,
    #[builder(setter(name = "_embed"), private, default)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<LevelEmbeds>,
}

/// Retrieves all categories for the given level.
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct LevelCategories<'a> {
    #[doc = r"`ID` of the level."]
    #[serde(skip)]
    id: LevelId<'a>,
    #[doc = r"When give, filters miscellaneous categories."]
    #[builder(default)]
    miscellaneous: Option<bool>,
    #[doc = r"Sorting options for results."]
    #[builder(default)]
    orderby: Option<CategoriesSorting>,
    #[doc = r"Sort direction"]
    #[builder(default)]
    direction: Option<Direction>,
}

/// Retrieves all applicable variables for the given level.
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct LevelVariables<'a> {
    #[doc = r"`ID` of the level."]
    #[serde(skip)]
    id: LevelId<'a>,
    #[doc = r"Sorting options for results."]
    #[builder(default)]
    orderby: Option<VariablesSorting>,
    #[doc = r"Sort direction"]
    #[builder(default)]
    direction: Option<Direction>,
}

/// Retrieves the leaderboards of the given level for all available categories.
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct LevelRecords<'a> {
    #[doc = r"`ID` of the level."]
    #[serde(skip)]
    id: LevelId<'a>,
    #[doc = r"Return `top` number of places (default: 3)."]
    #[builder(default)]
    top: Option<i64>,
    #[doc = r"Do not return empty leaderboards when `true`."]
    #[builder(default)]
    skip_empty: Option<bool>,
    #[builder(setter(name = "_embed"), private, default)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<LeaderboardEmbeds>,
}

impl Level<'_> {
    /// Create a builder for this endpoint.
    pub fn builder<'a>() -> LevelBuilder<'a> {
        LevelBuilder::default()
    }
}

impl LevelBuilder<'_> {
    /// Add an embedded resource to this result
    pub fn embed(&mut self, embed: LevelEmbeds) -> &mut Self {
        self.embed.get_or_insert_with(BTreeSet::new).insert(embed);
        self
    }

    /// Add multiple embedded resources to this result
    pub fn embeds<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = LevelEmbeds>,
    {
        self.embed.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }
}

impl LevelCategories<'_> {
    /// Create a builder for this endpoint.
    pub fn builder<'a>() -> LevelCategoriesBuilder<'a> {
        LevelCategoriesBuilder::default()
    }
}

impl LevelVariables<'_> {
    /// Create a builder for this endpoint.
    pub fn builder<'a>() -> LevelVariablesBuilder<'a> {
        LevelVariablesBuilder::default()
    }
}

impl LevelRecords<'_> {
    /// Create a builder for this endpoint.
    pub fn builder<'a>() -> LevelRecordsBuilder<'a> {
        LevelRecordsBuilder::default()
    }
}

impl LevelRecordsBuilder<'_> {
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

impl LevelEmbeds {
    fn as_str(&self) -> &'static str {
        match self {
            LevelEmbeds::Categories => "categories",
            LevelEmbeds::Variables => "variables",
        }
    }
}

impl Endpoint for Level<'_> {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/levels/{}", self.id).into()
    }

    fn query_parameters(&self) -> Result<QueryParams<'_>, BodyError> {
        QueryParams::with(self)
    }
}

impl Endpoint for LevelCategories<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/levels/{}/categories", self.id).into()
    }

    fn query_parameters(&self) -> Result<QueryParams<'_>, BodyError> {
        QueryParams::with(self)
    }
}

impl Endpoint for LevelVariables<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/levels/{}/variables", self.id).into()
    }

    fn query_parameters(&self) -> Result<QueryParams<'_>, BodyError> {
        QueryParams::with(self)
    }
}

impl Endpoint for LevelRecords<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/levels/{}/records", self.id).into()
    }

    fn query_parameters(&self) -> Result<QueryParams<'_>, BodyError> {
        QueryParams::with(self)
    }
}

impl From<&LevelEmbeds> for &'static str {
    fn from(value: &LevelEmbeds) -> Self {
        value.as_str()
    }
}

impl Pageable for LevelRecords<'_> {}
