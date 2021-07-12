use serde::{Deserialize, Serialize};

use crate::types::Pagination;

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum Direction {
    Asc,
    Desc,
}

/// Sorting options for variables
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum VariablesSorting {
    /// Sorts alphanumerically by the variable name
    Name,
    /// Sorts by `mandatory` flag
    Mandatory,
    /// Sorts by `user-defined` flag
    UserDefined,
    /// Sorts by the order defined by the game moderator (default)
    Pos,
}

/// Sorting options for categories
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum CategoriesSorting {
    /// Sort alphanumerically by category name
    Name,
    /// Sort by `miscellaneous` flag
    Miscellaneous,
    /// Use sort order defined by game moderator (default)
    Pos,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Root<T> {
    pub(crate) data: T,
    pub(crate) pagination: Option<Pagination>,
}

impl Default for VariablesSorting {
    fn default() -> Self {
        Self::Pos
    }
}

impl Default for CategoriesSorting {
    fn default() -> Self {
        Self::Pos
    }
}
