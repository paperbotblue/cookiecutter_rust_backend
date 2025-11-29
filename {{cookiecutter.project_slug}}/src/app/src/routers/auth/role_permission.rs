use actix_web::{
    middleware::from_fn,
    web::{self, ServiceConfig},
};
use auth::api::controllers::role_permission_handler::{
    create_role_permission_handler, delete_role_permission_handler, get_role_permission_handler,
    list_role_permissions_handler,
};
use auth::api::middlewares::jwt_extractor::check_permission_middleware;

pub fn role_permission_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/role_permissions") //
            .service(web::resource("").route(web::get().to(list_role_permissions_handler))),
    )
    .service(
        web::scope("/auth/role_permissions")
            .wrap(from_fn(|req, next| async move {
                check_permission_middleware(req, next, "admin").await
            }))
            .service(
                web::resource("")
                    .route(web::post().to(create_role_permission_handler))
                    .route(web::get().to(list_role_permissions_handler)),
            )
            .service(
                web::resource("/{role_id}/{permission_id}")
                    .route(web::get().to(get_role_permission_handler))
                    .route(web::delete().to(delete_role_permission_handler)),
            ),
    );
}
