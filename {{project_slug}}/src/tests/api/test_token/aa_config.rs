use std::str::FromStr;

use crate::tests::api::helper::Scope;
use chrono::{DateTime, Duration, Utc};
use cookiecutter_project_slug::{
    api::dto::{
        permission::{CreatePermissionDTO, PermissionDTO},
        token::{CreateTokenDTO, TokenDTO},
    },
    domain::{models::role::ClientType, repositories::repository::ResultPaging},
};
use once_cell::sync::Lazy;
use reqwest::Response;
use serde_json::{json, Value};
use uuid::Uuid;

pub static SCOPE: Scope = Scope::Tokens;

pub static CREATE_REQUEST_BODY: Lazy<Value> = Lazy::new(|| {
    json!({
        "client_id": "550e8400-e29b-41d4-a716-446655440000",
        "client_type": ClientType::User,
        "is_revoked": false,
        "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3NjU3NzE2MjksImlhdCI6MTc1NDk3MTYyOSwic3ViIjoiY2I1OTBjNzktYTU1NC00MjExLTg5ZmEtMzUzMWY2NTk5Y2ZhIiwicGVybWlzc2lvbnMiOiJ1c2VyIn0.ZOjzKMpnSyeZvclDp2pytsNf30M30qdmQKJ14OJqaGs",
        "expires_at": "2025-08-19T12:34:56.789Z",
        "created_at": "2025-08-19T12:34:56.789Z",
        "updated_at": "2025-08-19T12:34:56.789Z",
    })
});

pub static TEST_DATA_DTO: Lazy<CreateTokenDTO> = Lazy::new(|| {
    CreateTokenDTO {
    client_id: Uuid::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
    client_type: ClientType::User,
    is_revoked: false,
    token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3NjU3NzE2MjksImlhdCI6MTc1NDk3MTYyOSwic3ViIjoiY2I1OTBjNzktYTU1NC00MjExLTg5ZmEtMzUzMWY2NTk5Y2ZhIiwicGVybWlzc2lvbnMiOiJ1c2VyIn0.ZOjzKMpnSyeZvclDp2pytsNf30M30qdmQKJ14OJqaGs".to_string(),
    expires_at: "2025-08-19T12:34:56.789Z".parse::<DateTime<Utc>>().unwrap(),
    created_at: "2025-08-19T12:34:56.789Z".parse::<DateTime<Utc>>().unwrap(),
    updated_at: "2025-08-19T12:34:56.789Z".parse::<DateTime<Utc>>().unwrap(),
}
});

pub static CREATE_DUP_DATA: Lazy<Value> = Lazy::new(|| {
    json!({
        "client_id": "550e8400-e29b-41d4-a716-446655440011",
        "client_type": ClientType::User,
        "is_revoked": false,
        "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3NjU3NzE2MjksImlhdCI6MTc1NDk3MTYyOSwic3ViIjoiY2I1OTBjNzktYTU1NC00MjExLTg5ZmEtMzUzMWY2NTk5Y2ZhIiwicGVybWlzc2lvbnMiOiJ1c2VyIn0.ZOjzKMpnSyeZvclDp2pytsNf30M30qdmQKJ14OJqaGs",
        "expires_at": "2025-08-19T12:34:56.789Z",
        "created_at": "2025-08-19T12:34:56.789Z",
        "updated_at": "2025-08-19T12:34:56.789Z",
    })
});

pub static INVALID_DATA_NO_TOKEN: Lazy<Value> = Lazy::new(|| {
    json!({
        "client_id": "550e8400-e29b-41d4-a716-446655440099",
        "client_type": ClientType::User,
        "is_revoked": false,
        "token": "",
        "expires_at": "2025-08-19T12:34:56.789Z",
        "created_at": "2025-08-19T12:34:56.789Z",
        "updated_at": "2025-08-19T12:34:56.789Z",
    })
});

pub static INVALID_DATA_NO_CLIENT_TYPE: Lazy<Value> = Lazy::new(|| {
    json!({
        "client_id": "550e8400-e29b-41d4-a716-446655440099",
        "client_type": "",
        "is_revoked": false,
        "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3NjU3NzE2MjksImlhdCI6MTc1NDk3MTYyOSwic3ViIjoiY2I1OTBjNzktYTU1NC00MjExLTg5ZmEtMzUzMWY2NTk5Y2ZhIiwicGVybWlzc2lvbnMiOiJ1c2VyIn0.ZOjzKMpnSyeZvclDp2pytsNf30M30qdmQKJ14OJqaGs",
        "expires_at": "2025-08-19T12:34:56.789Z",
        "created_at": "2025-08-19T12:34:56.789Z",
        "updated_at": "2025-08-19T12:34:56.789Z",
    })
});

pub static INVALID_DATA_EMPTY_DATA: Lazy<Value> = Lazy::new(|| {
    json!({
        "client_id": "550e8400-e29b-41d4-a716-446655440099",
        "client_type": "",
        "is_revoked": false,
        "token": "",
        "expires_at": "2025-08-19T12:34:56.789Z",
        "created_at": "2025-08-19T12:34:56.789Z",
        "updated_at": "2025-08-19T12:34:56.789Z",
    })
});

pub fn get_pagination_data_10() -> Vec<Value> {
    let mut data: Vec<Value> = vec![];
    for i in 1..=10 {
        let request_body = json!({
        "client_id": Uuid::new_v4(),
        "client_type": ClientType::User,
        "is_revoked": false,
        "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3NjU3NzE2MjksImlhdCI6MTc1NDk3MTYyOSwic3ViIjoiY2I1OTBjNzktYTU1NC00MjExLTg5ZmEtMzUzMWY2NTk5Y2ZhIiwicGVybWlzc2lvbnMiOiJ1c2VyIn0.ZOjzKMpnSyeZvclDp2pytsNf30M30qdmQKJ14OJqaG".to_owned() + &i.to_string(),
        "expires_at": "2025-08-19T12:34:56.789Z",
        "created_at": "2025-08-19T12:34:56.789Z",
        "updated_at": "2025-08-19T12:34:56.789Z",
        });
        data.push(request_body);
    }
    data
}

pub async fn response_unwrap_dto(resp: Response) -> TokenDTO {
    resp.json().await.unwrap()
}

pub async fn response_unwrap_pagination(resp: Response) -> ResultPaging<TokenDTO> {
    resp.json().await.unwrap()
}
