#[derive(Debug, PartialEq, Eq, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub organization_id: OrganizationId,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OrganizationId(pub String);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Users {
    pub users: Vec<User>,
}
