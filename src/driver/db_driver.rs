use crate::domain::user::OrganizationId;

use super::model::UserModel;

#[mry::mry]
pub async fn find_users_for_organization_id(
    organizatoin_id: OrganizationId,
) -> anyhow::Result<Vec<UserModel>> {
    todo!()
}