#[cfg(test)]
mod test_item_crud {
    use cookiecutter_project_slug::domain::repositories::repository::ResultPaging;
    use reqwest::Client;
    use serde_json::json;

    use crate::tests::api::helper::{make_request, ApiRequest};
    use crate::tests::api::test_role::aa_config::{
        response_unwrap_dto, response_unwrap_pagination, CREATE_REQUEST_BODY, SCOPE, TEST_DATA_DTO,
    };

    #[tokio::test]
    async fn test_crud_operations() {
        let client = Client::new();

        // Create item
        let resp = make_request(
            &client,
            ApiRequest::Create(CREATE_REQUEST_BODY.clone()),
            &SCOPE,
        )
        .await;
        assert!(resp.status().is_success());

        let item = response_unwrap_dto(resp).await;
        assert_eq!(item.name, TEST_DATA_DTO.name);
        assert_eq!(item.description, TEST_DATA_DTO.description);

        // GET item with id
        let resp = make_request(&client, ApiRequest::GetOne(item.id.to_string()), &SCOPE).await;

        assert!(resp.status().is_success());
        let retrieved_item = response_unwrap_dto(resp).await;
        assert_eq!(item.id, retrieved_item.id);

        // List items
        let resp = make_request(&client, ApiRequest::GetAll, &SCOPE).await;

        assert!(resp.status().is_success());
        let items = response_unwrap_pagination(resp).await;
        assert_eq!(items.items.len(), 1);

        // Delete item
        let resp = make_request(&client, ApiRequest::Delete(item.id.to_string()), &SCOPE).await;
        assert!(resp.status().is_success());

        // Verify empty list
        let resp = make_request(&client, ApiRequest::GetAll, &SCOPE).await;
        assert!(resp.status().is_success());
        let items = response_unwrap_pagination(resp).await;
        assert_eq!(items.items.len(), 0);
    }
}
