use anyhow::{Ok, Result};

use connection_pool::DB_POOL;

use super::model::{MemberModel, OrganizationIdModel};
#[mry::mry]
pub async fn find_users_for_organization_id(
    organization_id: i32,
) -> anyhow::Result<Vec<MemberModel>> {
    let sql = "select * from users where organization_id = $1";

    let rows = sqlx::query_as::<_, MemberModel>(sql)
        .bind(organization_id)
        .fetch_all(DB_POOL.get().unwrap())
        .await?;
    Ok(rows)
}

#[mry::mry]
pub async fn add_user(name: String, organization_name: String) -> Result<MemberModel> {
    let organization_sql = "select id from organization where name = $1";

    let organization = sqlx::query_as::<_, OrganizationIdModel>(organization_sql)
        .bind(organization_name.clone())
        .fetch_one(DB_POOL.get().unwrap())
        .await?;

    let insert_sql = "insert into users (name, organization_id) values ($1, $2)";

    sqlx::query(insert_sql)
        .bind(name.clone())
        .bind(organization.0)
        .execute(DB_POOL.get().unwrap())
        .await?;

    let get_sql = "select * from users where name = $1";

    let row = sqlx::query_as::<_, MemberModel>(get_sql)
        .bind(name.clone())
        .fetch_one(DB_POOL.get().unwrap())
        .await?;

    Ok(row)
}

#[mry::mry]
pub async fn update_user(id: i32, name: String) -> Result<()> {
    let sql = "update users set name = $1 where id = $2";

    sqlx::query(sql)
        .bind(name.clone())
        .bind(id)
        .execute(DB_POOL.get().unwrap())
        .await?;
    Ok(())
}

#[mry::mry]
pub async fn delete_user(id: i32) -> Result<()> {
    let sql = "delete from users where id = $1";

    sqlx::query(sql)
        .bind(id.clone())
        .execute(DB_POOL.get().unwrap())
        .await?;
    Ok(())
}
