use anyhow::Ok;
use sqlx::{pool, postgres::PgPoolOptions};
use tracing::info;

use crate::domain::user::OrganizationId;

use super::model::UserModel;

#[mry::mry]
pub async fn find_users_for_organization_id(
    organization_id: OrganizationId,
) -> anyhow::Result<Vec<UserModel>> {
    let pool = PgPoolOptions::new()
        .connect("postgres://numa:password@localhost:5432/example")
        .await?;

    let sql = "select * from member where organization_id = $1";

    let rows = sqlx::query_as::<_, UserModel>(sql)
        .bind(organization_id.0)
        .fetch_all(&pool)
        .await?;
    Ok(rows)
}
