use crate::{
    domain::user::{OrganizationId, User, Users},
    port::user::MemberPort,
};

pub async fn get_users(id: OrganizationId, port: impl MemberPort) -> anyhow::Result<Users> {
    port.get_users_for_id(id).await
}

pub async fn add_user(
    name: String,
    organization_name: String,
    port: impl MemberPort,
) -> anyhow::Result<User> {
    port.add_user(name, organization_name).await
}

#[cfg(test)]
mod test {
    use anyhow::Ok;

    use crate::{
        domain::user::{OrganizationId, User},
        port::user::MockMemberPort,
    };

    use super::*;

    #[tokio::test]
    async fn test_get_users() {
        let expected = Users { users: vec![] };

        let id = OrganizationId(22);

        let mut member_port = MockMemberPort::default();
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

        let mut member_port = MockMemberPort::default();
        member_port
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
            member_port,
        )
        .await
        .unwrap();

        assert_eq!(expected, actual);
    }
}
