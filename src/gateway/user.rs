use crate::{
    domain::user::{OrganizationId, User, Users},
    driver,
    port::user::UserPort,
};

pub struct UserGateway;

#[async_trait::async_trait]
impl UserPort for UserGateway {
    async fn get_users_for_id(&self, organization_id: OrganizationId) -> anyhow::Result<Users> {
        let response = driver::db_driver::find_users_for_organization_id(organization_id).await?;

        Ok(Users {
            users: response
                .into_iter()
                .map(|user| User {
                    id: user.id,
                    name: user.name,
                    organization_id: OrganizationId(user.organization_id),
                })
                .collect(),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{
        domain::user::{OrganizationId, User, Users},
        driver::{self, db_driver::mock_find_users_for_organization_id, model::UserModel},
    };

    use super::*;

    #[tokio::test]
    #[mry::lock(driver::db_driver::find_users_for_organization_id)]
    async fn test_get_users() {
        let uuid = "C23DED40-F7BD-47B5-8A18-D2E5B48505A0".to_string();
        let organization_id = OrganizationId(uuid.clone());

        let expected = Users {
            users: vec![
                User {
                    id: 1,
                    name: "john".to_string(),
                    organization_id: organization_id.clone(),
                },
                User {
                    id: 2,
                    name: "andy".to_string(),
                    organization_id: organization_id.clone(),
                },
                User {
                    id: 3,
                    name: "numa".to_string(),
                    organization_id: organization_id.clone(),
                },
            ],
        };

        mock_find_users_for_organization_id(organization_id.clone()).returns_with(move |_| {
            Ok(vec![
                UserModel {
                    id: 1,
                    name: "john".to_string(),
                    organization_id: uuid.clone(),
                },
                UserModel {
                    id: 2,
                    name: "andy".to_string(),
                    organization_id: uuid.clone(),
                },
                UserModel {
                    id: 3,
                    name: "numa".to_string(),
                    organization_id: uuid.clone(),
                },
            ])
        });

        let actual = UserGateway.get_users_for_id(organization_id).await.unwrap();

        assert_eq!(expected, actual)
    }
}
