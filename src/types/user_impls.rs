use crate::api::users::UserId;

use super::User;

impl<'a> From<User<'a>> for UserId<'a> {
    fn from(value: User<'a>) -> Self {
        value.id
    }
}
