use std::str::FromStr;

use crate::tests::api::helper::Scope;
use cookiecutter_project_slug::{
    api::dto::{
        permission::{CreatePermissionDTO, PermissionDTO},
        role_permission::{CreateRolePermissionDTO, RolePermissionDTO},
    },
    domain::repositories::repository::ResultPaging,
};
use once_cell::sync::Lazy;
use reqwest::Response;
use serde_json::{json, Value};
use uuid::Uuid;

pub static SCOPE: Scope = Scope::RolePermissions;

pub static CREATE_REQUEST_BODY: Lazy<Value> = Lazy::new(|| {
    json!({
        "role_id": "550e8400-e29b-41d4-a716-446655440000",
        "permission_id":"550e8400-e29b-41d4-a716-446655440001",
        "description": "test body description"
    })
});

pub static TEST_DATA_DTO: Lazy<CreateRolePermissionDTO> = Lazy::new(|| CreateRolePermissionDTO {
    role_id: Uuid::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
    permission_id: Uuid::from_str("550e8400-e29b-41d4-a716-446655440001").unwrap(),
    description: "test body description".to_string(),
});

pub static CREATE_DUP_DATA: Lazy<Value> = Lazy::new(|| {
    json!({
        "role_id": Uuid::from_str("550e8400-e29b-41d4-a716-446655440010").unwrap(),
        "permission_id": Uuid::from_str("550e8400-e29b-41d4-a716-446655440011").unwrap(),
        "description": "test body description".to_string(),
    })
});

pub static INVALID_DATA_NO_ROLE_ID: Lazy<Value> = Lazy::new(|| {
    json!({
        "role_id": "",
        "permission_id": Uuid::from_str("550e8400-e29b-41d4-a716-446655440011").unwrap(),
        "description": "test body description".to_string(),
    })
});

pub static INVALID_DATA_NO_PERMISSION_ID: Lazy<Value> = Lazy::new(|| {
    json!({
        "role_id": Uuid::new_v4(),
        "permission_id": "",
        "description": "test body description".to_string(),
    })
});

pub static INVALID_DATA_NO_DESCRIPTION: Lazy<Value> = Lazy::new(|| {
    json!({
        "role_id": Uuid::new_v4(),
        "permission_id": Uuid::from_str("550e8400-e29b-41d4-a716-446655440011").unwrap(),
        "description": "".to_string(),
    })
});

pub static INVALID_DATA_EMPTY_DATA: Lazy<Value> = Lazy::new(|| {
    json!({
        "role_id": "",
        "permission_id": "",
        "description": "".to_string(),
    })
});

pub fn get_pagination_data_10() -> Vec<Value> {
    let mut data: Vec<Value> = vec![];
    for i in 1..=10 {
        let request_body = json!({
            "role_id": Uuid::new_v4(),
            "permission_id": Uuid::new_v4(),
            "description": format!("Description {}", i)
        });
        data.push(request_body);
    }
    data
}

pub async fn response_unwrap_dto(resp: Response) -> RolePermissionDTO {
    resp.json().await.unwrap()
}

pub async fn response_unwrap_pagination(resp: Response) -> ResultPaging<RolePermissionDTO> {
    resp.json().await.unwrap()
}
