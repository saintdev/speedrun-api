use std::borrow::Cow;

use http::Method;
use serde::Serialize;

use super::{endpoint::Endpoint, CategoriesSorting, Direction, Pageable, VariablesSorting};

#[derive(Default, Debug, Builder, Clone)]
#[builder(default, setter(into, strip_option))]
pub struct Level<'a> {
    id: Cow<'a, str>,
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct LevelCategories<'a> {
    #[serde(skip)]
    id: Cow<'a, str>,
    miscellaneous: Option<bool>,
    orderby: Option<CategoriesSorting>,
    direction: Option<Direction>,
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct LevelVariables<'a> {
    #[serde(skip)]
    id: Cow<'a, str>,
    orderby: Option<VariablesSorting>,
    direction: Option<Direction>,
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

impl<'a> Level<'a> {
    pub fn builder() -> LevelBuilder<'a> {
        LevelBuilder::default()
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

impl Endpoint for Level<'_> {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/levels/{}", self.id).into()
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

impl Pageable for LevelRecords<'_> {}
