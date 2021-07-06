use std::borrow::Cow;

use http::Method;
use serde::Serialize;

use super::{endpoint::Endpoint, Direction, Pageable};

#[derive(Default, Debug, Builder, Clone)]
#[builder(default, setter(into, strip_option))]
pub struct Level<'a> {
    id: Cow<'a, str>,
}

impl<'a> Level<'a> {
    pub fn builder() -> LevelBuilder<'a> {
        LevelBuilder::default()
    }
}

impl<'a> Endpoint for Level<'a> {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/levels/{}", self.id).into()
    }
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct LevelCategories<'a> {
    #[serde(skip)]
    id: Cow<'a, str>,
    miscellaneous: Option<bool>,
    orderby: Option<LevelCategoriesOrderBy>,
    direction: Option<Direction>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum LevelCategoriesOrderBy {
    Name,
    Miscellaneous,
    Pos,
}

impl<'a> LevelCategories<'a> {
    pub fn builder() -> LevelCategoriesBuilder<'a> {
        LevelCategoriesBuilder::default()
    }
}

impl<'a> Endpoint for LevelCategories<'a> {
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

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct LevelVariables<'a> {
    #[serde(skip)]
    id: Cow<'a, str>,
    orderby: Option<LevelVariablesOrderBy>,
    direction: Option<Direction>,
}

impl<'a> LevelVariables<'a> {
    pub fn builder() -> LevelVariablesBuilder<'a> {
        LevelVariablesBuilder::default()
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum LevelVariablesOrderBy {
    Name,
    Mandatory,
    UserDefined,
    Pos,
}

impl<'a> Endpoint for LevelVariables<'a> {
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

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct LevelRecords<'a> {
    #[serde(skip)]
    id: Cow<'a, str>,
    top: Option<i64>,
    skip_empty: Option<bool>,
}

impl<'a> LevelRecords<'a> {
    pub fn builder() -> LevelRecordsBuilder<'a> {
        LevelRecordsBuilder::default()
    }
}

impl<'a> Endpoint for LevelRecords<'a> {
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

impl Pageable for LevelRecords<'_> {}
