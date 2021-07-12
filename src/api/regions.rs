use std::borrow::Cow;

use http::Method;
use serde::Serialize;

use super::{endpoint::Endpoint, Direction, Pageable};

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Regions {
    direction: Option<Direction>,
}

#[derive(Default, Debug, Builder, Clone)]
#[builder(default, setter(into, strip_option))]
pub struct Region<'a> {
    id: Cow<'a, str>,
}

impl Regions {
    pub fn builder() -> RegionsBuilder {
        RegionsBuilder::default()
    }
}

impl<'a> Region<'a> {
    pub fn builder() -> RegionBuilder<'a> {
        RegionBuilder::default()
    }
}

impl Endpoint for Regions {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "/regions".into()
    }

    fn query_parameters(&self) -> Result<std::borrow::Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Endpoint for Region<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/regions/{}", self.id).into()
    }
}

impl Pageable for Regions {}
