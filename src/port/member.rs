use async_trait::async_trait;

use crate::domain::member::{Members, OrganizationId};

#[mry::mry]
#[async_trait]
pub trait UserPort {
    async fn get_users_for_id(&self, organization_id: OrganizationId) -> anyhow::Result<Members>;
}
