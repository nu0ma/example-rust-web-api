use crate::domain::user::User;

pub fn get_user_usecase() -> User {
    User {
        id: 1,
        name: String::from("test user"),
    }
}
