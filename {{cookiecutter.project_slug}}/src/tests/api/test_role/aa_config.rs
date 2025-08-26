use crate::tests::api::helper::Scope;
use cookiecutter_project_slug::{
    api::dto::permission::{CreatePermissionDTO, PermissionDTO},
    domain::repositories::repository::ResultPaging,
};
use once_cell::sync::Lazy;
use reqwest::Response;
use serde_json::{json, Value};

pub static SCOPE: Scope = Scope::Roles;

pub static CREATE_REQUEST_BODY: Lazy<Value> = Lazy::new(|| {
    json!({
        "name": "create_self",
        "description": "can create self"
    })
});

pub static TEST_DATA_DTO: Lazy<CreatePermissionDTO> = Lazy::new(|| CreatePermissionDTO {
    name: "create_self".to_string(),
    description: "can create self".to_string(),
});

pub static CREATE_DUP_DATA: Lazy<Value> = Lazy::new(|| {
    json!({
        "name": "test1",
        "description": "test1 permission"
    })
});

pub static INVALID_DATA_NO_NAME: Lazy<Value> = Lazy::new(|| {
    json!({
        "name": "",
        "description": "test1 permission"
    })
});

pub static INVALID_DATA_NO_DESCRIPTION: Lazy<Value> = Lazy::new(|| {
    json!({
        "name": "test1",
        "description": ""
    })
});

pub static INVALID_DATA_EMPTY_DATA: Lazy<Value> = Lazy::new(|| {
    json!({
        "name": "",
        "description": ""
    })
});

pub fn get_pagination_data_10() -> Vec<Value> {
    let mut data: Vec<Value> = vec![];
    for i in 1..=10 {
        let request_body = json!({
            "name": format!("permission_{}", i),
            "description": format!("Description {}", i)
        });
        data.push(request_body);
    }
    data
}

pub async fn response_unwrap_dto(resp: Response) -> PermissionDTO {
    resp.json().await.unwrap()
}

pub async fn response_unwrap_pagination(resp: Response) -> ResultPaging<PermissionDTO> {
    resp.json().await.unwrap()
}
