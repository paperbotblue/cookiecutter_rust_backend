use auth::api::controllers::user_handler::{
    create_user_handler, delete_user_handler, get_user_handler, get_user_profile_handler,
    list_users_handler, login_user_handler, logout_user_handler, send_otp_handler,
    update_user_handler, update_user_password_handler,
};

use actix_web::{
    middleware::from_fn,
    web::{self, ServiceConfig},
};

use auth::api::middlewares::jwt_extractor::check_permission_middleware;
pub fn user_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(
                web::resource("")
                    .route(web::post().to(create_user_handler))
                    .route(web::get().to(list_users_handler)),
            )
            .service(web::resource("/login").route(web::post().to(login_user_handler)))
            .service(web::resource("/logout").route(web::post().to(logout_user_handler)))
            .service(web::resource("/update").route(web::post().to(update_user_handler)))
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_user_handler))
                    .route(web::delete().to(delete_user_handler)),
            ),
    )
    .service(
        web::scope("/auth/users")
            .wrap(from_fn(|req, next| async move {
                check_permission_middleware(req, next, "users").await
            }))
            .service(
                web::resource("/update_password")
                    .route(web::post().to(update_user_password_handler)),
            )
            .service(web::resource("/send_otp").route(web::post().to(send_otp_handler)))
            .service(
                web::resource("/update_password")
                    .route(web::post().to(update_user_password_handler)),
            )
            .service(web::resource("/profile").route(web::get().to(get_user_profile_handler))),
    );
}
