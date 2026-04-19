use clap::Parser;
use tao::{cli, constants};

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_module("tao", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Off)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    let cli_args = cli::CliArgs::parse();

    let db_url: String = cli_args.database_url.or_else(|| std::env::var(constants::DB_URL_ENV_VAR).ok()).unwrap_or_else(|| {
        log::error!("Neither the \"--database-url\" argument was provided, nor the \"{}\" environment variable was set.", constants::DB_URL_ENV_VAR);
        std::process::exit(1);
    });

    let app_env = match std::env::var("PUBLIC_APP_ENV") {
        Ok(out) => out,
        Err(_) => {
            log::warn!("Unable to get the app environment, assuming development environment");
            String::from("development")
        }
    };

    let server_port: u16 = cli_args.port.parse().unwrap_or_else(|_| {
        log::error!("You must provide a valid port as input");
        std::process::exit(1);
    });

    match tao::run(&db_url, &app_env, server_port).await {
        Ok(_) => {}
        Err(e) => {
            log::error!("{}", e);
        }
    };
}
