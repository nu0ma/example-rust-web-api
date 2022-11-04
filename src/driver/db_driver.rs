use anyhow::Ok;
use sqlx::postgres::PgPoolOptions;

use crate::{domain::member::OrganizationId, utils::connection_pool::DB_POOL};

use super::model::MemberModel;

#[mry::mry]
pub async fn find_users_for_organization_id(
    organization_id: OrganizationId,
) -> anyhow::Result<Vec<MemberModel>> {
    let sql = "select * from member where organization_id = $1";

    let rows = sqlx::query_as::<_, MemberModel>(sql)
        .bind(organization_id.0)
        .fetch_all(DB_POOL.get().unwrap())
        .await?;
    Ok(rows)
}
