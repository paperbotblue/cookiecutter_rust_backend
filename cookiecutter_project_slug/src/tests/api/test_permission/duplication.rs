#[cfg(test)]
mod test_item_duplication {

    use reqwest::Client;
    use serde_json::json;
    use uuid::Uuid;

    use crate::tests::api::{
        helper::{make_request, ApiRequest},
        test_permission::aa_config::{response_unwrap_dto, CREATE_DUP_DATA, SCOPE},
    };

    #[tokio::test]
    async fn test_duplicated_input_name() {
        let client = Client::new();

        let resp = make_request(&client, ApiRequest::Create(CREATE_DUP_DATA.clone()), &SCOPE).await;
        let status = resp.status();

        let item = response_unwrap_dto(resp).await;
        eprintln!("Unable to create item: Status {}", status);
        assert_eq!(status, 200);

        let resp = make_request(&client, ApiRequest::Create(CREATE_DUP_DATA.clone()), &SCOPE).await;
        let status = resp.status();
        eprintln!("Unable to create item: Status {}", status);
        assert_eq!(status, 409);

        let resp = make_request(&client, ApiRequest::Delete(item.id.to_string()), &SCOPE).await;
        assert!(resp.status().is_success());
    }
}
