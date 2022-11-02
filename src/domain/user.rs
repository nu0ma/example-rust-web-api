#[derive(Debug, PartialEq, Eq, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub age: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]

pub struct Id(pub u32);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Users {
    pub users: Vec<User>,
}
