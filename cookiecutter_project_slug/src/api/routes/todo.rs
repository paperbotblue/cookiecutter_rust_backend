use crate::api::controllers::todo_handler::{
    create_todo_handler,
    update_todo_handler,
    delete_todo_handler,
    get_todo_handler,
    list_todos_handler,
};

use actix_web::{
    middleware::from_fn,
    web::{self, ServiceConfig},
};

pub fn todo_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            // Uncomment if you need a middleware for this scope
            // .wrap(from_fn(|req, next| async move {
            //     check_permission_middleware(req, next, "admin").await
            // }))
            .service(
                web::resource("")
                    .route(web::post().to(create_todo_handler))
                    .route(web::get().to(list_todos_handler)),
            )
            .service(web::resource("/update").route(web::post().to(update_todo_handler)))
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_todo_handler))
                    .route(web::delete().to(delete_todo_handler)),
            ),
    );
}
