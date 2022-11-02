use async_trait::async_trait;

use crate::domain::user::{Id, Users};

#[mry::mry]
#[async_trait]
pub trait UserPort {
    async fn get_users_for_id(&self, id: Id) -> anyhow::Result<Users>;
}
