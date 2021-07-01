use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum Direction {
    Asc,
    Desc,
}
