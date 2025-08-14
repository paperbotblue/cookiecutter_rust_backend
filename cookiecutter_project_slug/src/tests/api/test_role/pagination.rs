#[cfg(test)]
mod test_role_crud {
    use cookiecutter_project_slug::api::dto::role::RoleDTO;

    use cookiecutter_project_slug::domain::repositories::repository::ResultPaging;
    use reqwest::Client;
    use serde_json::json;

    use crate::tests::api::setup::spawn_app;
    use crate::tests::api::test_role::helper::{
        create_role, delete_role_by_id, list_paginated_roles,
    };

    #[tokio::test]
    async fn test_pagination() {
        let base_url = spawn_app().await;
        let client = Client::new();

        // Create 10 roles
        for i in 1..=10 {
            let request_body = json!({
                "name": format!("Role {}", i),
                "description": format!("Description {}", i)
            });

            let resp = create_role(&client, request_body).await;
            assert!(resp.status().is_success());
        }

        let resp = list_paginated_roles(&client, 5, 0).await;

        assert!(resp.status().is_success());
        let roles: ResultPaging<RoleDTO> = resp.json().await.unwrap();
        assert_eq!(roles.items.len(), 5);

        let resp = list_paginated_roles(&client, 10, 0).await;

        assert!(resp.status().is_success());
        let roles: ResultPaging<RoleDTO> = resp.json().await.unwrap();
        assert_eq!(roles.items.len(), 10);

        for role in roles.items {
            let resp = delete_role_by_id(&client, role.id).await;

            assert!(resp.status().is_success());
        }
    }
}
