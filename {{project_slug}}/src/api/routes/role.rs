use crate::api::controllers::role_handler::{
    create_role_handler, delete_role_handler, get_role_handler, list_roles_handler,
    update_role_handler,
};
use actix_web::web;

pub fn role_scope() -> actix_web::Scope {
    web::scope("/roles")
        .route("", web::post().to(create_role_handler))
        .route("/update", web::post().to(update_role_handler))
        .route("", web::get().to(list_roles_handler))
        .route("/{id}", web::get().to(get_role_handler))
        .route("/{id}", web::delete().to(delete_role_handler))
}
