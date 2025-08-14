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

    async fn test_invalid_input_name() {
        let base_url = spawn_app().await;
        let client = Client::new();

        let request_body = json!({
            "name": "",
            "description": "Invalid role"
        });
        let resp = client
            .post(format!("{}/roles", base_url))
            .json(&request_body)
            .send()
            .await
            .unwrap();
        let status = resp.status();
        let body_str = resp.text().await.unwrap();
        eprintln!(
            "Invalid input response: Status {}, Response: {}",
            status, body_str
        );
        assert_eq!(status, 400);
    }

    async fn test_invalid_input_description() {
        let base_url = spawn_app().await;
        let client = Client::new();

        let request_body = json!({
            "name": "subadmin",
            "description": ""
        });
        let resp = client
            .post(format!("{}/roles", base_url))
            .json(&request_body)
            .send()
            .await
            .unwrap();
        let status = resp.status();
        let body_str = resp.text().await.unwrap();
        eprintln!(
            "Invalid input response: Status {}, Response: {}",
            status, body_str
        );
        assert_eq!(status, 400);
    }

    async fn test_invalid_input_name_and_description() {
        let base_url = spawn_app().await;
        let client = Client::new();

        let request_body = json!({
            "name": "",
            "description": ""
        });
        let resp = client
            .post(format!("{}/roles", base_url))
            .json(&request_body)
            .send()
            .await
            .unwrap();
        let status = resp.status();
        let body_str = resp.text().await.unwrap();
        eprintln!(
            "Invalid input response: Status {}, Response: {}",
            status, body_str
        );
        assert_eq!(status, 400);
    }

    async fn test_non_existent_role_retrieval() {
        let base_url = spawn_app().await;
        let client = Client::new();

        let resp = client
            .get(format!("{}/roles/9999", base_url))
            .send()
            .await
            .unwrap();
        let status = resp.status();
        let body_str = resp.text().await.unwrap();
        eprintln!(
            "Non-existent role response: Status {}, Response: {}",
            status, body_str
        );
        assert_eq!(status, 404);
    }
}
