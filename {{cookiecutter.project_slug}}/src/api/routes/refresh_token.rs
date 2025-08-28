use crate::api::{
    controllers::refresh_token_handler::{
        create_refresh_token_handler, delete_refresh_token_handler, get_refresh_token_handler,
        list_refresh_tokens_handler, renew_refresh_token_handler, update_refresh_token_handler,
    },
    middlewares::jwt_extractor::check_permission_middleware,
};

use actix_web::{
    middleware::from_fn,
    web::{self, ServiceConfig},
};

pub fn refresh_token_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/refresh_tokens")
            .wrap(from_fn(|req, next| async move {
                check_permission_middleware(req, next, "admin").await
            }))
            .service(
                web::resource("")
                    .route(web::post().to(create_refresh_token_handler))
                    .route(web::get().to(list_refresh_tokens_handler)),
            )
            .service(web::resource("/update").route(web::post().to(update_refresh_token_handler)))
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_refresh_token_handler))
                    .route(web::delete().to(delete_refresh_token_handler)),
            ),
    )
    .service(
        web::scope("/token")
            .service(web::resource("/refresh").route(web::post().to(renew_refresh_token_handler))),
    );
}
