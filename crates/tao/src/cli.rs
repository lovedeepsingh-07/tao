use crate::constants;

#[derive(Debug, clap::Parser)]
#[command(version)]
pub struct CliArgs {
    #[arg(long, default_value=constants::DEFAULT_SERVER_PORT, global = true)]
    /// Server port
    pub port: String,
    #[arg(long, global = true)]
    /// Postgres database url, instead of passing the url as an argument, you can also set the
    /// "TAO_DB_URL" environment variable if security is a concern
    pub database_url: Option<String>,
}
