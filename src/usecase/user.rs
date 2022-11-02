use crate::{
    domain::user::{OrganizationId, User, Users},
    port::user::UserPort,
};

pub async fn get_users(id: OrganizationId, port: impl UserPort) -> anyhow::Result<Users> {
    port.get_users_for_id(id).await
}

#[cfg(test)]
mod test {
    use crate::{
        domain::user::{OrganizationId, User},
        port::user::MockUserPort,
    };

    use super::*;

    #[tokio::test]
    async fn test_get_users() {
        let expected = Users { users: vec![] };

        let id = OrganizationId("uuid1".to_string());

        let mut user_port = MockUserPort::default();
        user_port
            .mock_get_users_for_id(id.clone())
            .returns_with(|_| Ok(Users { users: vec![] }));

        let actual = get_users(id, user_port).await.unwrap();

        assert_eq!(expected, actual);
    }
}
