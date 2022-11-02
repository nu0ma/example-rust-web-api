use async_trait::async_trait;

use crate::domain::user::{OrganizationId, Users};

#[mry::mry]
#[async_trait]
pub trait UserPort {
    async fn get_users_for_id(&self, organization_id: OrganizationId) -> anyhow::Result<Users>;
}
