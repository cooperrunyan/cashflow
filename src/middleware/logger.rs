use std::env;

use actix_web::middleware::Logger;
use env_logger::{Builder, Target};

pub fn logger() -> Logger {
    Logger::new(r#"%r %T %s"#)
}

pub fn init_logger() {
    Builder::new()
        .format_module_path(false)
        .format_level(true)
        .format_target(false)
        .format_indent(Some(8))
        .filter_level(log::LevelFilter::Warn)
        .parse_env("log_level")
        .target(Target::Stdout)
        .write_style(env_logger::WriteStyle::Always)
        .format_timestamp(None)
        .init();

    if env::var("log_level").is_err() {
        warn!("Variable 'log_level' not set, defaulting to log_level=WARN");
    }
}
