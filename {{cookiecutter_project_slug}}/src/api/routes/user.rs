use crate::api::controllers::user_handler::{
    create_user_handler, delete_user_handler, get_user_handler, list_users_handler,
    update_user_handler,
};

use actix_web::{
    // middleware::from_fn,
    web::{self, ServiceConfig},
};

pub fn user_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/users")
            // Uncomment if you need a middleware for this scope
            // .wrap(from_fn(|req, next| async move {
            //     check_permission_middleware(req, next, "admin").await
            // }))
            .service(
                web::resource("")
                    .route(web::post().to(create_user_handler))
                    .route(web::get().to(list_users_handler)),
            )
            .service(web::resource("/update").route(web::post().to(update_user_handler)))
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_user_handler))
                    .route(web::delete().to(delete_user_handler)),
            ),
    );
}
