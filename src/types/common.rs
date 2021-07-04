use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Link {
    pub rel: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Pagination {
    pub offset: usize,
    pub max: usize,
    pub size: usize,
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimingMethod {
    Realtime,
    RealtimeNoloads,
    Ingame,
}
