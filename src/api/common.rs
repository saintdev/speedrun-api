use serde::Serialize;

/// Sort direction
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum Direction {
    /// Sort ascending
    Asc,
    /// Sort descending
    Desc,
}

/// Sorting options for variables
#[derive(Default, Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum VariablesSorting {
    /// Sorts alphanumerically by the variable name
    Name,
    /// Sorts by `mandatory` flag
    Mandatory,
    /// Sorts by `user-defined` flag
    UserDefined,
    /// Sorts by the order defined by the game moderator (default)
    #[default]
    Pos,
}

/// Sorting options for categories
#[derive(Default, Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum CategoriesSorting {
    /// Sort alphanumerically by category name
    Name,
    /// Sort by `miscellaneous` flag
    Miscellaneous,
    /// Use sort order defined by game moderator (default)
    #[default]
    Pos,
}
