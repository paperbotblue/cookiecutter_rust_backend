#[cfg(test)]
mod test_todo_controllers {
    use actix_web::test;
    use cookiecutter_project_slug::api::dto::role::RoleDTO;
    use cookiecutter_project_slug::domain::constants::POSTGRESQL_DB_URI;
    use cookiecutter_project_slug::domain::models::role::Role;
    use cookiecutter_project_slug::domain::repositories::repository::ResultPaging;
    use cookiecutter_project_slug::infrastructure::databases::postgresql::db_pool;
    use cookiecutter_project_slug::{container::Container, create_app::create_app};
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use serde_json::json;
    use std::env;
    use std::sync::Arc;
    use testcontainers::clients;
    use testcontainers::images::postgres;
    use tokio::sync::OnceCell;

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    // Static container for test environment, initialized once
    static TEST_ENV: OnceCell<Arc<Container>> = OnceCell::const_new();

    // Helper function to set up test environment
    async fn setup_test_env() -> Arc<Container> {
        env::set_var("RUST_BACKTRACE", "1");
        env::set_var("RUST_LOG", "debug");
        env_logger::init();

        let docker = clients::Cli::default();
        let postgres_node = docker.run(postgres::Postgres::default());
        let connection_string = &format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            postgres_node.get_host_port_ipv4(5432)
        );

        env::set_var(POSTGRESQL_DB_URI, connection_string);

        let pool = Arc::new(db_pool());
        pool.get()
            .unwrap()
            .run_pending_migrations(MIGRATIONS)
            .unwrap();

        Arc::new(Container::new())
    }

    // Helper to get or initialize the test environment
    async fn get_test_env() -> &'static Arc<Container> {
        TEST_ENV.get_or_init(setup_test_env).await
    }

    #[actix_web::test]
    async fn test_crud_operations() {
        let container = get_test_env().await;
        let app = test::init_service(create_app(container.clone())).await;

        let request_body = json!({
            "name": "Admin",
            "description": "Admin auth level 2"
        });

        // Creation test
        let req = test::TestRequest::post()
            .uri("/roles")
            .set_json(&request_body)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let role: RoleDTO = test::read_body_json(resp).await;
        assert_eq!(role.name, "Admin");
        assert_eq!(role.description, "Admin auth level 2");

        // Get single role test
        let req = test::TestRequest::get()
            .uri(&format!("/roles/{}", role.id))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let retrieved_role: RoleDTO = test::read_body_json(resp).await;
        assert_eq!(role.id, retrieved_role.id);
        assert_eq!(role.name, retrieved_role.name);

        // Creation test (second role)
        let req = test::TestRequest::post()
            .uri("/roles")
            .set_json(&request_body)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Get all roles test
        let req = test::TestRequest::get().uri("/roles").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let roles: ResultPaging<RoleDTO> = test::read_body_json(resp).await;
        assert_eq!(roles.items.len(), 2);

        // Delete test
        let req = test::TestRequest::delete()
            .uri(&format!("/roles/{}", role.id))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Get all roles test after deletion
        let req = test::TestRequest::get().uri("/roles").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let roles: ResultPaging<RoleDTO> = test::read_body_json(resp).await;
        assert_eq!(roles.items.len(), 1);
    }

    #[actix_web::test]
    async fn test_invalid_input() {
        let container = get_test_env().await;
        let app = test::init_service(create_app(container.clone())).await;

        // Test with empty name
        let request_body = json!({
            "name": "",
            "description": "Invalid role"
        });
        let req = test::TestRequest::post()
            .uri("/roles")
            .set_json(&request_body)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400); // Expect Bad Request
    }

    #[actix_web::test]
    async fn test_non_existent_role_retrieval() {
        let container = get_test_env().await;
        let app = test::init_service(create_app(container.clone())).await;

        // Test retrieving a non-existent role
        let req = test::TestRequest::get().uri("/roles/9999").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404); // Expect Not Found
    }

    #[actix_web::test]
    async fn test_pagination() {
        let container = get_test_env().await;
        let app = test::init_service(create_app(container.clone())).await;

        // Create multiple roles
        for i in 1..=10 {
            let request_body = json!({
                "name": format!("Role {}", i),
                "description": format!("Description {}", i)
            });
            let req = test::TestRequest::post()
                .uri("/roles")
                .set_json(&request_body)
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success());
        }

        // Test pagination
        let req = test::TestRequest::get()
            .uri("/roles?page=1&size=5")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let roles: ResultPaging<RoleDTO> = test::read_body_json(resp).await;
        assert_eq!(roles.items.len(), 5); // Expect 5 items per page
    }
}
