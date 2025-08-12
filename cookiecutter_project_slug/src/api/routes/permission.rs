use crate::api::controllers::permission_handler::{
    create_permission_handler, delete_permission_handler, get_permission_handler,
    list_permissions_handler, update_permission_handler,
};
use actix_web::web;

pub fn permission_scope() -> actix_web::Scope {
    web::scope("/permissions")
        .route("", web::post().to(create_permission_handler))
        .route("/update", web::post().to(update_permission_handler))
        .route("", web::get().to(list_permissions_handler))
        .route("/{id}", web::get().to(get_permission_handler))
        .route("/{id}", web::delete().to(delete_permission_handler))
}
