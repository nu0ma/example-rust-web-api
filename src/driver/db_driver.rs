use anyhow::Ok;
use sqlx::postgres::PgPoolOptions;

use crate::domain::member::OrganizationId;

use super::model::MemberModel;

#[mry::mry]
pub async fn find_users_for_organization_id(
    organization_id: OrganizationId,
) -> anyhow::Result<Vec<MemberModel>> {
    let pool = PgPoolOptions::new()
        .connect("postgres://numa:password@localhost:5432/example")
        .await?;

    let sql = "select * from member where organization_id = $1";

    let rows = sqlx::query_as::<_, MemberModel>(sql)
        .bind(organization_id.0)
        .fetch_all(&pool)
        .await?;
    Ok(rows)
}
