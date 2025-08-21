#[cfg(test)]
mod test_item_pagination {
    use cookiecutter_project_slug::domain::repositories::repository::ResultPaging;
    use reqwest::Client;
    use serde_json::json;

    use crate::tests::api::helper::{make_request, ApiRequest};
    use crate::tests::api::setup::spawn_app;
    use crate::tests::api::test_token::aa_config::{
        get_pagination_data_10, response_unwrap_pagination, SCOPE,
    };

    #[tokio::test]
    async fn test_pagination() {
        let base_url = spawn_app().await;
        let client = Client::new();

        let request_bodies = get_pagination_data_10();
        for request_body in request_bodies {
            let resp = make_request(&client, ApiRequest::Create(request_body), &SCOPE).await;
            assert!(resp.status().is_success());
        }

        let resp = make_request(&client, ApiRequest::GetAllPaginated(0, 5), &SCOPE).await;

        assert!(resp.status().is_success());
        let items = response_unwrap_pagination(resp).await;
        assert_eq!(items.items.len(), 5);

        let resp = make_request(&client, ApiRequest::GetAllPaginated(0, 10), &SCOPE).await;

        assert!(resp.status().is_success());
        let items = response_unwrap_pagination(resp).await;
        assert_eq!(items.items.len(), 10);

        for item in items.items {
            let resp = make_request(&client, ApiRequest::Delete(item.id.to_string()), &SCOPE).await;

            assert!(resp.status().is_success());
        }
    }
}
