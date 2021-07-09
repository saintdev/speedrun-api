use std::collections::HashMap;

use serde::Deserialize;

use super::Link;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Variable {
    pub id: String,
    pub name: String,
    pub category: Option<String>,
    pub scope: Scope,
    pub mandatory: bool,
    pub user_defined: bool,
    pub obsoletes: bool,
    pub values: Values,
    pub is_subcategory: bool,
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type")]
pub enum Scope {
    Global,
    FullGame,
    AllLevels,
    SingleLevel { level: String },
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Values {
    pub values: HashMap<String, Value>,
    pub default: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Value {
    pub label: String,
    pub rules: Option<String>,
    pub flags: Option<Flags>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Flags {
    pub miscellaneous: Option<bool>,
}
