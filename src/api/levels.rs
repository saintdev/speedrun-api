use std::{borrow::Cow, collections::BTreeSet, fmt::Display};

use http::Method;
use serde::{Deserialize, Serialize};

use super::{
    endpoint::Endpoint, leaderboards::LeaderboardEmbeds, CategoriesSorting, Direction, Pageable,
    VariablesSorting,
};

/// Embeds available for levels.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LevelEmbeds {
    Categories,
    Variables,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LevelId<'a>(Cow<'a, str>);

impl<'a> LevelId<'a> {
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

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
pub struct Level<'a> {
    #[serde(skip)]
    id: LevelId<'a>,
    #[builder(setter(name = "_embed"), private, default)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<LevelEmbeds>,
}

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct LevelCategories<'a> {
    #[serde(skip)]
    id: LevelId<'a>,
    #[builder(default)]
    miscellaneous: Option<bool>,
    #[builder(default)]
    orderby: Option<CategoriesSorting>,
    #[builder(default)]
    direction: Option<Direction>,
}

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct LevelVariables<'a> {
    #[serde(skip)]
    id: LevelId<'a>,
    #[builder(default)]
    orderby: Option<VariablesSorting>,
    #[builder(default)]
    direction: Option<Direction>,
}

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct LevelRecords<'a> {
    #[serde(skip)]
    id: LevelId<'a>,
    #[builder(default)]
    top: Option<i64>,
    #[builder(default)]
    skip_empty: Option<bool>,
    #[builder(setter(name = "_embed"), private, default)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<LeaderboardEmbeds>,
}

impl<'a> Level<'a> {
    pub fn builder() -> LevelBuilder<'a> {
        LevelBuilder::default()
    }
}

impl<'a> LevelBuilder<'a> {
    pub fn embed(&mut self, embed: LevelEmbeds) -> &mut Self {
        self.embed.get_or_insert_with(BTreeSet::new).insert(embed);
        self
    }

    pub fn embeds<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = LevelEmbeds>,
    {
        self.embed.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }
}

impl<'a> LevelCategories<'a> {
    pub fn builder() -> LevelCategoriesBuilder<'a> {
        LevelCategoriesBuilder::default()
    }
}

impl<'a> LevelVariables<'a> {
    pub fn builder() -> LevelVariablesBuilder<'a> {
        LevelVariablesBuilder::default()
    }
}

impl<'a> LevelRecords<'a> {
    pub fn builder() -> LevelRecordsBuilder<'a> {
        LevelRecordsBuilder::default()
    }
}

impl<'a> LevelRecordsBuilder<'a> {
    pub fn embed(&mut self, embed: LeaderboardEmbeds) -> &mut Self {
        self.embed.get_or_insert_with(BTreeSet::new).insert(embed);
        self
    }

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

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Endpoint for LevelCategories<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/levels/{}/categories", self.id).into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Endpoint for LevelVariables<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/levels/{}/variables", self.id).into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Endpoint for LevelRecords<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/levels/{}/records", self.id).into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl From<&LevelEmbeds> for &'static str {
    fn from(value: &LevelEmbeds) -> Self {
        value.as_str()
    }
}

impl Pageable for LevelRecords<'_> {}
