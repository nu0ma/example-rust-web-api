use anyhow::Ok;

use crate::{
    domain::user::{OrganizationId, User, Users},
    driver::{self, db_driver},
    port::user::UserPort,
};

pub struct UserGateway;

#[async_trait::async_trait]
impl UserPort for UserGateway {
    async fn get_users_for_id(&self, organization_id: OrganizationId) -> anyhow::Result<Users> {
        let response = driver::db_driver::find_users_for_organization_id(organization_id.0).await?;

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

    async fn add_user(&self, name: String, organization_name: String) -> anyhow::Result<User> {
        let response = db_driver::add_user(name, organization_name).await?;
        Ok(User {
            id: response.id,
            name: response.name,
            organization_id: OrganizationId(response.organization_id),
        })
    }

    async fn update_user(&self, id: i32, name: String) -> anyhow::Result<()> {
        Ok(db_driver::update_user(id, name).await?)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        domain::user::{OrganizationId, User, Users},
        driver::{
            self,
            db_driver::{
                self, mock_add_user, mock_find_users_for_organization_id, mock_update_user,
            },
            model::MemberModel,
        },
    };

    use super::*;

    #[tokio::test]
    #[mry::lock(driver::db_driver::find_users_for_organization_id)]
    async fn test_get_users() {
        let organization_id = OrganizationId(1);

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

        mock_find_users_for_organization_id(organization_id.0.clone()).returns_with(move |_| {
            Ok(vec![
                MemberModel {
                    id: 1,
                    name: "john".to_string(),
                    organization_id: 1,
                },
                MemberModel {
                    id: 2,
                    name: "andy".to_string(),
                    organization_id: 1,
                },
                MemberModel {
                    id: 3,
                    name: "numa".to_string(),
                    organization_id: 1,
                },
            ])
        });

        let actual = UserGateway.get_users_for_id(organization_id).await.unwrap();

        assert_eq!(expected, actual)
    }

    #[tokio::test]
    #[mry::lock(db_driver::add_user)]
    async fn test_add_user() {
        let expected = User {
            id: 1,
            name: "test_user".to_string(),
            organization_id: OrganizationId(1),
        };

        mock_add_user(
            "test_user".to_string(),
            "test_organization_name".to_string(),
        )
        .returns_with(|_, _| {
            Ok(MemberModel {
                id: 1,
                name: "test_user".to_string(),
                organization_id: 1,
            })
        });

        let actual = UserGateway
            .add_user(
                "test_user".to_string(),
                "test_organization_name".to_string(),
            )
            .await
            .unwrap();

        assert_eq!(expected, actual);
    }

    #[tokio::test]
    #[mry::lock(db_driver::update_user)]
    async fn test_update_user() {
        let id = 1;
        let name = "updated".to_string();
        let expected = Ok(()).unwrap();

        mock_update_user(id, name.clone()).returns_with(|_, _| Ok(()));

        let actual = UserGateway.update_user(id, name).await.unwrap();

        assert_eq!(expected, actual);
    }
}
