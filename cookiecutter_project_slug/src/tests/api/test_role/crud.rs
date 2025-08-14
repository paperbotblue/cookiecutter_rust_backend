#[cfg(test)]
mod test_role_crud {
    use cookiecutter_project_slug::api::dto::role::RoleDTO;

    use cookiecutter_project_slug::domain::repositories::repository::ResultPaging;
    use reqwest::Client;
    use serde_json::json;

    use crate::tests::api::test_role::helper::{create_role, delete_role_by_id, get_role_by_id, list_roles};

    #[tokio::test]
    async fn test_crud_operations() {
        let client = Client::new();

        // Create role
        let request_body = json!({
            "name": "Admin",
            "description": "Admin auth level 2"
        });
        let resp = create_role(&client, request_body).await;

        assert!(resp.status().is_success());
        let role: RoleDTO = resp.json().await.unwrap();
        assert_eq!(role.name, "Admin");
        assert_eq!(role.description, "Admin auth level 2");

        // Get role by id
        let resp = get_role_by_id(&client, role.id).await;

        assert!(resp.status().is_success());
        let retrieved_role: RoleDTO = resp.json().await.unwrap();
        assert_eq!(role.id, retrieved_role.id);

        // List roles
        let resp = list_roles(&client).await;
        assert!(resp.status().is_success());
        let roles: ResultPaging<RoleDTO> = resp.json().await.unwrap();
        assert_eq!(roles.items.len(), 1);

        // Delete role
        let resp = delete_role_by_id(&client, role.id).await;
        assert!(resp.status().is_success());

        // Verify empty list
        let resp = list_roles(&client).await;
        assert!(resp.status().is_success());
        let roles: ResultPaging<RoleDTO> = resp.json().await.unwrap();
        assert_eq!(roles.items.len(), 0);
    }
}
