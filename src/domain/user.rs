use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Member {
    pub id: i32,
    pub name: String,
    pub organization_id: OrganizationId,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct OrganizationId(pub i32);

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Users {
    pub users: Vec<Member>,
}
