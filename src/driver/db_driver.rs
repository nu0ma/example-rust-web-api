use crate::domain::user::OrganizationId;

use super::model::UserModel;

#[mry::mry]
pub async fn find_users_for_organization_id(
    organization_id: OrganizationId,
) -> anyhow::Result<Vec<UserModel>> {
    todo!()
}
