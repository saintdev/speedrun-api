use serde::{Deserialize, Serialize};

use crate::types::Pagination;

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum Direction {
    Asc,
    Desc,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Root<T> {
    pub(crate) data: T,
    pub(crate) pagination: Option<Pagination>,
}
