use crate::container::Container;
use crate::routers::auth;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::{web, App, Error};
use base::log_config::log_format_config;
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
    let permission_service = container.permission_service.clone();
    let role_service = container.role_service.clone();
    let role_permission_service = container.role_permission_service.clone();
    let user_service = container.user_service.clone();
    let refresh_token_service = container.refresh_token_service.clone();
    // let school_service = container.school_service.clone();
    // let user_address_service = container.user_address_service.clone();
    // let user_bank_detail_service = container.user_bank_detail_service.clone();
    // let caste_service = container.caste_service.clone();
    // let religion_service = container.religion_service.clone();
    // let department_service = container.department_service.clone();
    // let designation_service = container.designation_service.clone();
    // let staff_service = container.staff_service.clone();
    // let attendance_service = container.attendance_service.clone();
    // let leave_request_service = container.leave_request_service.clone();
    // let leave_type_service = container.leave_type_service.clone();
    // let student_service = container.student_service.clone();
    // let class_service = container.class_service.clone();
    //
    let logger = log_format_config();

    App::new()
        .app_data(web::Data::from(permission_service))
        .app_data(web::Data::from(role_service))
        .app_data(web::Data::from(role_permission_service))
        .app_data(web::Data::from(user_service))
        .app_data(web::Data::from(refresh_token_service))
        // .app_data(web::Data::from(school_service))
        // .app_data(web::Data::from(user_address_service))
        // .app_data(web::Data::from(user_bank_detail_service))
        // .app_data(web::Data::from(caste_service))
        // .app_data(web::Data::from(religion_service))
        // .app_data(web::Data::from(department_service))
        // .app_data(web::Data::from(designation_service))
        // .app_data(web::Data::from(staff_service))
        // .app_data(web::Data::from(attendance_service))
        // .app_data(web::Data::from(leave_type_service))
        // .app_data(web::Data::from(leave_request_service))
        // .app_data(web::Data::from(student_service))
        // .app_data(web::Data::from(class_service))
        .wrap(logger)
        .configure(auth::auth_routes)
    // .configure(permission_config)
    // .configure(user_config)
    // .configure(role_permission_config)
    // .configure(role_config)
    // .configure(school_config)
    // .configure(user_address_config)
    // .configure(user_bank_detail_config)
    // .configure(refresh_token_config)
    // .configure(caste_config)
    // .configure(religion_config)
    // .configure(department_config)
    // .configure(designation_config)
    // .configure(staff_config)
    // .configure(attendance_config)
    // .configure(leave_type_config)
    // .configure(leave_request_config)
    // .configure(student_config)
    // .configure(class_config)
    // .configure(mail_notify_config)
    // .configure(digiischool_config)
}
