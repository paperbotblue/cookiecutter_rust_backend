#[cfg(test)]
mod test_role_crud {
    use cookiecutter_project_slug::api::dto::role::RoleDTO;
    use cookiecutter_project_slug::container::Container;
    use cookiecutter_project_slug::create_app::create_app;
    use cookiecutter_project_slug::domain::repositories::repository::ResultPaging;
    use reqwest::Client;
    use serde_json::json;
    use std::net::TcpListener;
    use std::sync::Arc;
    use tokio::task;

    use crate::tests::api::setup::{setup_test_env, spawn_app};

    async fn test_crud_operations() {
        let base_url = spawn_app().await;
        let client = Client::new();

        // Create role
        let request_body = json!({
            "name": "Admin",
            "description": "Admin auth level 2"
        });
        let resp = client
            .post(format!("{}/roles", base_url))
            .json(&request_body)
            .send()
            .await
            .unwrap();
        assert!(resp.status().is_success());
        let role: RoleDTO = resp.json().await.unwrap();
        assert_eq!(role.name, "Admin");
        assert_eq!(role.description, "Admin auth level 2");

        // Get role by id
        let resp = client
            .get(format!("{}/roles/{}", base_url, role.id))
            .send()
            .await
            .unwrap();
        assert!(resp.status().is_success());
        let retrieved_role: RoleDTO = resp.json().await.unwrap();
        assert_eq!(role.id, retrieved_role.id);

        // List roles
        let resp = client
            .get(format!("{}/roles", base_url))
            .send()
            .await
            .unwrap();
        assert!(resp.status().is_success());
        let roles: ResultPaging<RoleDTO> = resp.json().await.unwrap();
        assert_eq!(roles.items.len(), 1);

        // Delete role
        let resp = client
            .delete(format!("{}/roles/{}", base_url, role.id))
            .send()
            .await
            .unwrap();
        assert!(resp.status().is_success());

        // Verify empty list
        let resp = client
            .get(format!("{}/roles", base_url))
            .send()
            .await
            .unwrap();
        assert!(resp.status().is_success());
        let roles: ResultPaging<RoleDTO> = resp.json().await.unwrap();
        assert_eq!(roles.items.len(), 0);
    }
}
