use actix_web::HttpServer;
use cookiecutterproject_slug::{
    container::Container, create_app::create_app, domain::constants,
    utils::log_config::config_logger,
};

use std::sync::Arc;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config_logger();
    let port = *constants::PORT;
    let address = (*constants::ADDRESS).clone();

    let container = Arc::new(Container::new());
    let server = HttpServer::new(move || create_app(container.clone())).bind((address, port))?;
    server.run().await
}
