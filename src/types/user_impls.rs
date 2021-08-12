use std::fmt::Display;

use crate::api::users::UserId;

use super::User;

impl<'a> From<User<'a>> for UserId<'a> {
    fn from(value: User<'a>) -> Self {
        value.id
    }
}

impl Display for User<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.names.international)
    }
}
