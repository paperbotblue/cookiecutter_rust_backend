use cookiecutter_project_slug::api::dto::role::RoleDTO;
use cookiecutter_project_slug::container::Container;
use cookiecutter_project_slug::create_app::create_app;
use cookiecutter_project_slug::domain::repositories::repository::ResultPaging;
use reqwest::{Client, Response};
use serde_json::{json, Value};
use std::net::TcpListener;
use std::sync::Arc;
use tokio::task;
use uuid::Uuid;

use crate::tests::api::setup::spawn_app;

pub async fn create_role(client: &Client, request_body: Value) -> Response {
    let base_url = spawn_app().await;
    client
        .post(format!("{}/roles", base_url))
        .json(&request_body)
        .send()
        .await
        .unwrap()
}

pub async fn update_role(client: &Client, request_body: Value) -> Response {
    let base_url = spawn_app().await;
    client
        .put(format!("{}/roles", base_url))
        .json(&request_body)
        .send()
        .await
        .unwrap()
}

pub async fn get_role_by_id(client: &Client, id: Uuid) -> Response {
    let base_url = spawn_app().await;
    client
        .get(format!("{}/roles/{}", base_url, id))
        .send()
        .await
        .unwrap()
}

pub async fn list_roles(client: &Client) -> Response {
    let base_url = spawn_app().await;
    client
        .get(format!("{}/roles", base_url))
        .send()
        .await
        .unwrap()
}

pub async fn delete_role_by_id(client: &Client, id: Uuid) -> Response {
    let base_url = spawn_app().await;
    client
        .delete(format!("{}/roles/{}", base_url, id))
        .send()
        .await
        .unwrap()
}
