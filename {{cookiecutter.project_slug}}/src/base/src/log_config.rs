use log::Level;
use nu_ansi_term::Color;
use std::io::Write;

use actix_web::middleware::Logger;
use chrono::{FixedOffset, Utc};

pub fn config_logger() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let level = match record.level() {
                Level::Error => Color::Red.paint("ERROR"),
                Level::Warn => Color::Yellow.paint("WARN"),
                Level::Info => Color::Green.paint("INFO"),
                Level::Debug => Color::Blue.paint("DEBUG"),
                Level::Trace => Color::Purple.paint("TRACE"),
            };
            writeln!(buf, "{}: {}", level, record.args())
        })
        .init();
}

pub fn log_format_config() -> Logger {
    let ist_offset = FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap();
    let logger_format = "%{ist_time}xi %{X-Forwarded-For}i %r %s %b %{Referer}i %{User-Agent}i %T";

    Logger::new(logger_format).custom_request_replace("ist_time", move |_req| {
        Utc::now()
            .with_timezone(&ist_offset)
            .format("%Y-%m-%d %H:%M:%S %:z")
            .to_string()
    })
}
