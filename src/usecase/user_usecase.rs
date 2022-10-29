use crate::{domain::user::User, gateway::user_gateway::get_user};

pub fn get_user_usecase() -> User {
    get_user()
}
