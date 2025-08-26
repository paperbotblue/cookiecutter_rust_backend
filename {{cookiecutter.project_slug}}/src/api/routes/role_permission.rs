use crate::api::controllers::role_permission_handler::{
    create_role_permission_handler, delete_role_permission_handler, get_role_permission_handler,
    list_role_permissions_handler,
};
use actix_web::web;

pub fn role_permission_scope() -> actix_web::Scope {
    web::scope("/role_permissions")
        .route("", web::post().to(create_role_permission_handler))
        .route("", web::get().to(list_role_permissions_handler))
        .route("/{id}", web::get().to(get_role_permission_handler))
        .route("/{id}", web::delete().to(delete_role_permission_handler))
}
