pub fn get_user_items() -> UserJson {
    UserJson {
        id: 2,
        name: String::from("test user 2"),
        age: 300,
    }
}

pub struct UserJson {
    pub id: i32,
    pub name: String,
    pub age: i32,
}
