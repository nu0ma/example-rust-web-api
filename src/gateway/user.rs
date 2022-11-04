use crate::{
    domain::user::{Member, OrganizationId, Users},
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
                .map(|user| Member {
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
        domain::user::{Member, OrganizationId, Users},
        driver::{self, db_driver::mock_find_users_for_organization_id, model::UserModel},
    };

    use super::*;

    #[tokio::test]
    #[mry::lock(driver::db_driver::find_users_for_organization_id)]
    async fn test_get_users() {
        let organization_id = OrganizationId(1);

        let expected = Users {
            users: vec![
                Member {
                    id: 1,
                    name: "john".to_string(),
                    organization_id: organization_id.clone(),
                },
                Member {
                    id: 2,
                    name: "andy".to_string(),
                    organization_id: organization_id.clone(),
                },
                Member {
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
                    organization_id: 1,
                },
                UserModel {
                    id: 2,
                    name: "andy".to_string(),
                    organization_id: 1,
                },
                UserModel {
                    id: 3,
                    name: "numa".to_string(),
                    organization_id: 1,
                },
            ])
        });

        let actual = UserGateway.get_users_for_id(organization_id).await.unwrap();

        assert_eq!(expected, actual)
    }
}
