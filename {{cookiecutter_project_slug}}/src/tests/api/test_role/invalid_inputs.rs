#[cfg(test)]
mod test_item_invalid_inputs {

    use reqwest::Client;
    use serde_json::json;
    use uuid::Uuid;

    use crate::tests::api::{
        helper::{make_request, ApiRequest},
        test_role::aa_config::{
            INVALID_DATA_EMPTY_DATA, INVALID_DATA_NO_DESCRIPTION, INVALID_DATA_NO_NAME, SCOPE,
        },
    };

    #[tokio::test]
    async fn test_invalid_input_name() {
        let client = Client::new();

        let resp = make_request(
            &client,
            ApiRequest::Create(INVALID_DATA_NO_NAME.clone()),
            &SCOPE,
        )
        .await;

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

        let resp = make_request(
            &client,
            ApiRequest::Create(INVALID_DATA_NO_DESCRIPTION.clone()),
            &SCOPE,
        )
        .await;
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

        let resp = make_request(
            &client,
            ApiRequest::Create(INVALID_DATA_EMPTY_DATA.clone()),
            &SCOPE,
        )
        .await;

        let status = resp.status();
        let body_str = resp.text().await.unwrap();
        eprintln!(
            "Invalid input response: Status {}, Response: {}",
            status, body_str
        );
        assert_eq!(status, 400);
    }

    #[tokio::test]
    async fn test_non_existent_item_retrieval() {
        let client = Client::new();
        let id = Uuid::new_v4().to_string();

        let resp = make_request(&client, ApiRequest::GetOne(id), &SCOPE).await;
        let status = resp.status();
        let body_str = resp.text().await.unwrap();
        eprintln!(
            "Non-existent item response: Status {}, Response: {}",
            status, body_str
        );
        assert_eq!(status, 404);
    }
    #[tokio::test]
    async fn test_invalid_id_item_retrieval() {
        let client = Client::new();
        let id = "29302932".to_string();

        let resp = make_request(&client, ApiRequest::GetOne(id), &SCOPE).await;
        let status = resp.status();
        let body_str = resp.text().await.unwrap();
        eprintln!(
            "Non-existent item response: Status {}, Response: {}",
            status, body_str
        );
        assert_eq!(status, 400);
    }

    #[tokio::test]
    async fn test_invalid_id_delete() {
        let client = Client::new();
        let id = "29302932".to_string();

        let resp = make_request(&client, ApiRequest::Delete(id), &SCOPE).await;
        let status = resp.status();
        let body_str = resp.text().await.unwrap();
        eprintln!(
            "Non-existent item response: Status {}, Response: {}",
            status, body_str
        );
        assert_eq!(status, 400);
    }
}
