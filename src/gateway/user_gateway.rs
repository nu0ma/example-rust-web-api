use crate::{domain::user::User, driver::user_driver::get_user_items};

pub fn get_user() -> User {
    let response = get_user_items();
    User {
        id: response.id,
        name: response.name,
    }
}
