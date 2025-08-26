use crate::api::routes::permission::permission_config;
use crate::api::routes::role::role_scope;
use crate::api::routes::todo::todo_config;
use crate::api::routes::user::user_config;
use crate::container::Container;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::Error;
use actix_web::{web, App};
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
    let todo_service = container.todo_service.clone();
    let permission_service = container.permission_service.clone();
    let role_service = container.role_service.clone();
    let role_permission_service = container.role_permission_service.clone();
    let user_service = container.user_service.clone();

    App::new()
        .app_data(web::Data::from(permission_service.clone()))
        .app_data(web::Data::from(role_service.clone()))
        .app_data(web::Data::from(role_permission_service.clone()))
        .app_data(web::Data::from(todo_service.clone()))
        .app_data(web::Data::from(user_service.clone()))
        .wrap(Logger::default())
        .configure(permission_config)
        .configure(user_config)
        .configure(todo_config)
        .service(role_scope())
    //.service(todo_scope())
}
