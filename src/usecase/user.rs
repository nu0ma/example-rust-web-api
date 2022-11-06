use crate::{
    domain::user::{OrganizationId, User, Users},
    port::user::UserPort,
};

pub async fn get_users(id: OrganizationId, port: impl UserPort) -> anyhow::Result<Users> {
    port.get_users_for_id(id).await
}

pub async fn add_user(
    name: String,
    organization_name: String,
    port: impl UserPort,
) -> anyhow::Result<User> {
    port.add_user(name, organization_name).await
}

pub async fn update_user(id: i32, port: impl UserPort) -> anyhow::Result<()> {
    port.update_user(id).await
}

#[cfg(test)]
mod test {
    use anyhow::Ok;

    use crate::{
        domain::user::{OrganizationId, User},
        port::user::MockUserPort,
    };

    use super::*;

    #[tokio::test]
    async fn test_get_users() {
        let expected = Users { users: vec![] };

        let id = OrganizationId(22);

        let mut member_port = MockUserPort::default();
        member_port
            .mock_get_users_for_id(id.clone())
            .returns_with(|_| Ok(Users { users: vec![] }));

        let actual = get_users(id, member_port).await.unwrap();

        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn test_add_user() {
        let expected = User {
            id: 1,
            name: "created_user".to_string(),
            organization_id: OrganizationId(1),
        };

        let mut user_port = MockUserPort::default();
        user_port
            .mock_add_user(
                "created_user".to_string(),
                "test_organization_name".to_string(),
            )
            .returns_with(move |_, _| {
                Ok(User {
                    id: 1,
                    name: "created_user".to_string(),
                    organization_id: OrganizationId(1),
                })
            });

        let actual = add_user(
            "created_user".to_string(),
            "test_organization_name".to_string(),
            user_port,
        )
        .await
        .unwrap();

        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn test_update_user() {
        let id = 1;

        let mut user_port = MockUserPort::default();

        user_port.mock_update_user(id).returns_with(|_| Ok(()));

        let expected = Ok(()).unwrap();
        let actual = update_user(id, user_port).await.unwrap();

        assert_eq!(expected, actual);
    }
}
