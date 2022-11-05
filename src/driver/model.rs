use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct MemberModel {
    pub id: i32,
    pub name: String,
    pub organization_id: i32,
}

#[derive(FromRow)]
pub struct OrganizationIdModel(pub i32);
