use actix_web::test;
use actix_web::{body::MessageBody, dev::ServiceResponse};
use cookiecutter_project_slug::container::Container;
use cookiecutter_project_slug::create_app::create_app;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use once_cell::sync::OnceCell;
use std::net::TcpListener;
use std::process::Command;
use std::sync::Arc;
use testcontainers::{clients, images::postgres};
use tokio::task;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct TestEnv {
    pub container: Arc<Container>,
    pub _docker_container: testcontainers::Container<'static, postgres::Postgres>,
}

// This stores the base URL after the first spawn
pub static BASE_URL: OnceCell<String> = OnceCell::new();

pub fn setup_test_env() -> TestEnv {
    let docker = Box::leak(Box::new(clients::Cli::default()));
    let docker_container = docker.run(postgres::Postgres::default());
    let port = docker_container.get_host_port_ipv4(5432);
    let conn_str = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);
    std::env::set_var("POSTGRESQL_DB_URI", &conn_str);

    let app_container = Arc::new(Container::new());

    TestEnv {
        container: app_container,
        _docker_container: docker_container,
    }
}

pub async fn spawn_app() -> String {
    if let Some(url) = BASE_URL.get() {
        return url.clone();
    }

    // Otherwise, start the app
    let env = setup_test_env();
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();
    let server = actix_web::HttpServer::new(move || create_app(env.container.clone()))
        .listen(listener)
        .unwrap()
        .run();

    task::spawn(server); // spawn server in background
    let url = format!("http://127.0.0.1:{}", port);

    // Store URL so future calls reuse it
    BASE_URL
        .set(url.clone())
        .map_err(|err| println!("+++++++++++++++++ {:?}", err));
    url
}
