use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct UserModel {
    pub id: i32,
    pub name: String,
    pub organization_id: i32,
}
