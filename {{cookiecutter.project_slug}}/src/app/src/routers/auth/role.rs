use actix_web::{
    middleware::from_fn,
    web::{self, ServiceConfig},
};
use auth::api::controllers::role_handler::{
    create_role_handler, delete_role_handler, get_role_handler, list_roles_handler,
    update_role_handler,
};

use auth::api::middlewares::jwt_extractor::check_permission_middleware;

pub fn role_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/roles")
            .service(web::resource("").route(web::get().to(list_roles_handler)))
            .service(web::resource("/{id}").route(web::get().to(get_role_handler))),
    )
    .service(
        web::scope("/auth/roles")
            .wrap(from_fn(|req, next| async move {
                check_permission_middleware(req, next, "admin").await
            }))
            .service(web::resource("").route(web::post().to(create_role_handler)))
            .service(web::resource("/update").route(web::post().to(update_role_handler)))
            .service(web::resource("/{id}").route(web::delete().to(delete_role_handler))),
    );
}
