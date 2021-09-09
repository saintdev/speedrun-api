//! # Notifications
//!
//! Endpoints available for notifications.
use http::Method;
use serde::Serialize;

use super::{endpoint::Endpoint, Direction};

/// Sorting options for notifications
#[derive(Debug, Clone, Serialize, Copy)]
pub enum NotificationsSorting {
    /// Sort by the date the notification was created (default)
    Created,
}

/// Retrieves the notifications for the currently authenticated user.
#[derive(Default, Debug, Builder, Clone, Serialize)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Notifications {
    #[doc = r"Sorting options for results (default: Sorted by date)."]
    orderby: Option<NotificationsSorting>,
    #[doc = r"Sort direction. (default: descending)"]
    direction: Option<Direction>,
}

impl Notifications {
    /// Create a builder for this endpoint.
    pub fn builder() -> NotificationsBuilder {
        NotificationsBuilder::default()
    }
}

impl Default for NotificationsSorting {
    fn default() -> Self {
        Self::Created
    }
}

impl Endpoint for Notifications {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "notifications".into()
    }

    fn query_parameters(&self) -> Result<std::borrow::Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }

    fn requires_authentication(&self) -> bool {
        true
    }
}
