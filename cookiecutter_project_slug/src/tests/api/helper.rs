use std::fmt::{self, Display};

use reqwest::{Client, Response};
use serde_json::Value;
use uuid::Uuid;

use crate::tests::api::setup::spawn_app;

pub enum Scope {
    Roles,
    Permissions,
    RolePermissions,
    Tokens,
}

impl Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Scope::Roles => "roles",
            Scope::Permissions => "permissions",
            Scope::RolePermissions => "role_permissions",
            Scope::Tokens => "tokens",
        };
        write!(f, "{}", s)
    }
}

pub enum ApiRequest {
    Create(Value),
    Update(Value),
    GetOne(String),
    GetAll,

    /// ( offset , limit )
    GetAllPaginated(i32, i32),
    Delete(String),
}

pub async fn make_request(client: &Client, request_type: ApiRequest, scope: &Scope) -> Response {
    let base_url = spawn_app().await;
    match request_type {
        ApiRequest::Create(body) => {
            let url = format!("{}/{}", base_url, scope);
            client.post(url).json(&body).send().await.unwrap()
        }
        ApiRequest::Update(body) => {
            let url = format!("{}/{}", base_url, scope);
            client.post(url).json(&body).send().await.unwrap()
        }
        ApiRequest::GetOne(id) => {
            let url = format!("{}/{}/{}", base_url, scope, id);
            client.get(url).send().await.unwrap()
        }
        ApiRequest::GetAll => {
            let url = format!("{}/{}", base_url, scope);
            client.get(url).send().await.unwrap()
        }
        ApiRequest::GetAllPaginated(offset, limit) => {
            let url = format!(
                "{}/{}?offset={}&limit={}&title=test",
                base_url, scope, offset, limit
            );
            client.get(url).send().await.unwrap()
        }
        ApiRequest::Delete(id) => {
            let url = format!("{}/{}/{}", base_url, scope, id);
            client.delete(url).send().await.unwrap()
        }
    }
}
