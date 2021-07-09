use serde::Deserialize;

use super::{Link, Names};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct User {
    pub id: String,
    pub names: Names,
    pub pronouns: Option<String>,
    pub weblink: String,
    pub name_style: NameStyle,
    pub role: UserRole,
    pub signup: Option<String>,
    pub location: Option<Location>,
    pub twitch: Option<BasicLink>,
    pub hitbox: Option<BasicLink>,
    pub youtube: Option<BasicLink>,
    pub twitter: Option<BasicLink>,
    pub speedrunslive: Option<BasicLink>,
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "style")]
pub enum NameStyle {
    Solid {
        color: Color,
    },
    #[serde(rename_all = "kebab-case")]
    Gradient {
        color_from: Color,
        color_to: Color,
    },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UserRole {
    Banned,
    User,
    Trusted,
    Moderator,
    Admin,
    Programmer,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Color {
    pub light: String,
    pub dark: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Location {
    pub country: Place,
    pub region: Option<Place>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Place {
    pub code: String,
    pub names: Names,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BasicLink {
    pub uri: String,
}
