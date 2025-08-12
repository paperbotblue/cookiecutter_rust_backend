use actix_web::HttpServer;
use cookiecutter_project_slug::{container::Container, create_app::create_app};
use std::sync::Arc;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let container = Arc::new(Container::new());
    let server =
        HttpServer::new(move || create_app(container.clone())).bind(("127.0.0.1", 8080))?;
    server.run().await
}
