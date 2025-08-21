use actix_web::{
    middleware::from_fn,
    web::{self, ServiceConfig},
};

use crate::api::controllers::permission_handler::{
    create_permission_handler, delete_permission_handler, get_permission_handler,
    list_permissions_handler, update_permission_handler,
};

pub fn permission_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/permissions")
            // Uncomment if you need a middleware for this scope
            // .wrap(from_fn(|req, next| async move {
            //     check_permission_middleware(req, next, "admin").await
            // }))
            .service(
                web::resource("")
                    .route(web::post().to(create_permission_handler))
                    .route(web::get().to(list_permissions_handler)),
            )
            .service(web::resource("/update").route(web::post().to(update_permission_handler)))
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_permission_handler))
                    .route(web::delete().to(delete_permission_handler)),
            ),
    );
}
