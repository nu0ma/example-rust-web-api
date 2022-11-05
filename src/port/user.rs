use async_trait::async_trait;

use crate::domain::user::{OrganizationId, User, Users};

#[mry::mry]
#[async_trait]
pub trait MemberPort {
    async fn get_users_for_id(&self, organization_id: OrganizationId) -> anyhow::Result<Users>;
    async fn add_user(&self, name: String, organization_name: String) -> anyhow::Result<User>;
}
