#[cfg(test)]
mod test_role_crud {

    use reqwest::Client;
    use serde_json::json;
    use uuid::Uuid;

    use crate::tests::api::test_role::helper::{create_role, get_role_by_id};

    #[tokio::test]
    async fn test_invalid_input_name() {
        let client = Client::new();

        let request_body = json!({
            "name": "",
            "description": "Invalid role"
        });

        let resp = create_role(&client, request_body).await;

        let status = resp.status();
        let body_str = resp.text().await.unwrap();
        eprintln!(
            "Invalid input response: Status {}, Response: {}",
            status, body_str
        );
        assert_eq!(status, 400);
    }

    #[tokio::test]
    async fn test_invalid_input_description() {
        let client = Client::new();

        let request_body = json!({
            "name": "subadmin",
            "description": ""
        });
        let resp = create_role(&client, request_body).await;
        let status = resp.status();
        let body_str = resp.text().await.unwrap();
        eprintln!(
            "Invalid input response: Status {}, Response: {}",
            status, body_str
        );
        assert_eq!(status, 400);
    }

    #[tokio::test]
    async fn test_invalid_input_name_and_description() {
        let client = Client::new();

        let request_body = json!({
            "name": "",
            "description": ""
        });
        let resp = create_role(&client, request_body).await;

        let status = resp.status();
        let body_str = resp.text().await.unwrap();
        eprintln!(
            "Invalid input response: Status {}, Response: {}",
            status, body_str
        );
        assert_eq!(status, 400);
    }

    #[tokio::test]
    async fn test_non_existent_role_retrieval() {
        let client = Client::new();

        let resp = get_role_by_id(&client, Uuid::new_v4()).await;
        let status = resp.status();
        let body_str = resp.text().await.unwrap();
        eprintln!(
            "Non-existent role response: Status {}, Response: {}",
            status, body_str
        );
        assert_eq!(status, 404);
    }
    #[tokio::test]
    async fn test_invalid_id_role_retrieval() {
        let client = Client::new();

        let resp = get_role_by_id(&client, "233223").await;
        let status = resp.status();
        let body_str = resp.text().await.unwrap();
        eprintln!(
            "Non-existent role response: Status {}, Response: {}",
            status, body_str
        );
        assert_eq!(status, 400);
    }
}
