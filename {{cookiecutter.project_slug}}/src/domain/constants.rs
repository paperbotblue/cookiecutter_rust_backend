use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref DATABASE_URL: String = set_db_url();
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
    pub static ref JWT_SECRET: String = set_jwt_secret();
    pub static ref REFRESH_TOKEN_COOKIE_NAME: String = set_refresh_token_cookie_name();
    pub static ref REFRESH_TOKEN_SECRET: String = set_refresh_token_secret();
    pub static ref REFRESH_TOKEN_EXP_DAYS: i64 = set_refresh_token_exp_days();
    pub static ref JWT_TOKEN_EXP_MINUTES: i64 = set_jwt_token_exp_minutes();
    pub static ref SMTP_HOST: String = set_smtp_host();
    pub static ref SMTP_EMAIL: String = set_smtp_email();
    pub static ref SMTP_PASSWORD: String = set_smtp_password();
}

fn set_smtp_host() -> String {
    dotenv::dotenv().ok();
    env::var("SMTP_HOST").expect("No smtp host set")
}

fn set_smtp_email() -> String {
    dotenv::dotenv().ok();
    env::var("SMTP_MAIL").expect("No smtp email set")
}

fn set_smtp_password() -> String {
    dotenv::dotenv().ok();
    env::var("SMTP_PASSWORD").expect("No smtp password set")
}

fn set_address() -> String {
    dotenv::dotenv().ok();
    env::var("ADDRESS").expect("address not set")
}

fn set_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("PORT")
        .unwrap()
        .parse::<u16>()
        .expect("Can't parse the port")
}

fn set_db_url() -> String {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL constant not found in env")
}

fn set_jwt_secret() -> String {
    dotenv::dotenv().ok();
    env::var("JWT_SECRET").expect("JWT Secret not found in env")
}

fn set_refresh_token_secret() -> String {
    dotenv::dotenv().ok();
    env::var("REFRESH_TOKEN_SECRET").expect("Refresh Token Secret not found in env")
}
fn set_refresh_token_cookie_name() -> String {
    dotenv::dotenv().ok();
    env::var("REFRESH_TOKEN_COOKIE_NAME").expect("Refresh Token cookie name not found in env")
}

fn set_refresh_token_exp_days() -> i64 {
    dotenv::dotenv().ok();
    env::var("REFRESH_TOKEN_EXP_DAYS")
        .unwrap()
        .parse::<i64>()
        .expect("refresh token exp not set")
}

fn set_jwt_token_exp_minutes() -> i64 {
    dotenv::dotenv().ok();
    env::var("JWT_TOKEN_EXP_MINUTES")
        .unwrap()
        .parse::<i64>()
        .expect("Jwt token exp not set")
}
