use crate::container::Container;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::App;
use actix_web::Error;
use std::sync::Arc;

pub fn create_app(
    container: Arc<Container>,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    //let todo_service = container.todo_service.clone();

    App::new()
        //.app_data(web::Data::from(todo_service.clone()))
        .wrap(Logger::default())
    //.service(todo_scope())
}
