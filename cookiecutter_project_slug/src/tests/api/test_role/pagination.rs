#[cfg(test)]
mod test_role_crud {
    use cookiecutter_project_slug::api::dto::role::RoleDTO;
    use cookiecutter_project_slug::container::Container;
    use cookiecutter_project_slug::create_app::create_app;
    use cookiecutter_project_slug::domain::repositories::repository::ResultPaging;
    use reqwest::Client;
    use serde_json::{json, Value};
    use std::net::TcpListener;
    use std::sync::Arc;
    use tokio::task;

    use crate::tests::api::setup::{setup_test_env, spawn_app};

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

            let resp = client
                .post(format!("{}/roles", base_url))
                .json(&request_body)
                .send()
                .await
                .unwrap();
            assert!(resp.status().is_success());
        }

        let resp = client
            .get(format!("{}/roles?limit=5&offset=0&title=hero", base_url))
            .send()
            .await
            .unwrap();
        assert!(resp.status().is_success());
        let roles: ResultPaging<RoleDTO> = resp.json().await.unwrap();
        assert_eq!(roles.items.len(), 5);

        let resp = client
            .get(format!("{}/roles?limit=10&offset=0&title=hero", base_url))
            .send()
            .await
            .unwrap();
        assert!(resp.status().is_success());
        let roles: ResultPaging<RoleDTO> = resp.json().await.unwrap();
        assert_eq!(roles.items.len(), 10);

        for role in roles.items {
            let resp = client
                .delete(format!("{}/roles/{}", base_url, role.id))
                .send()
                .await
                .unwrap();

            assert!(resp.status().is_success());
        }
    }
}
