use crate::api::controllers::token_handler::{
    create_refresh_token_handler, delete_refresh_token_handler, get_refresh_token_handler,
    list_refresh_tokens_handler, update_refresh_token_handler,
};
use actix_web::web;

pub fn refresh_token_scope() -> actix_web::Scope {
    web::scope("/refresh_token")
        .route("", web::post().to(create_refresh_token_handler))
        .route("/update", web::post().to(update_refresh_token_handler))
        .route("", web::get().to(list_refresh_tokens_handler))
        .route("/{id}", web::get().to(get_refresh_token_handler))
        .route("/{id}", web::delete().to(delete_refresh_token_handler))
}
