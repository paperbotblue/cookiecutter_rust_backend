use actix_web::{
    middleware::from_fn,
    web::{self, ServiceConfig},
};
use auth::api::controllers::refresh_token_handler::{
    create_refresh_token_handler, delete_refresh_token_handler, get_refresh_token_handler,
    list_refresh_tokens_handler, renew_refresh_token_handler, revoke_refresh_tokens_handler,
    update_refresh_token_handler,
};

use auth::api::middlewares::jwt_extractor::check_permission_middleware;

pub fn refresh_token_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/refresh_tokens")
            .service(web::resource("/renew").route(web::post().to(renew_refresh_token_handler)))
            .service(
                web::resource("/revoke/{id}").route(web::post().to(revoke_refresh_tokens_handler)),
            ),
    )
    .service(
        web::scope("/auth/refresh_tokens")
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
    );
}
