use permission::permission_config;
use refresh_token::refresh_token_config;
use role::role_config;
use role_permission::role_permission_config;
use user::user_config;

use actix_web::web::ServiceConfig;
pub fn auth_routes(cfg: &mut ServiceConfig) {
    user_config(cfg);
    permission_config(cfg);
    role_config(cfg);
    role_permission_config(cfg);
    refresh_token_config(cfg);
}

mod permission;
mod refresh_token;
mod role;
mod role_permission;
mod user;
