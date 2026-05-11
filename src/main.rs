use clap::Parser;
use goldfish::run;
use goldfish::settings::Settings;
use std::net::IpAddr;

#[derive(Parser)]
#[command(version, about = "Concurrent memcached implementation in Rust")]
struct Cli {
    #[arg(long, help = "IP address to bind to [default: 0.0.0.0]")]
    host: Option<IpAddr>,

    #[arg(long, help = "Port to listen on [default: 11211]")]
    port: Option<u16>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut env = env_logger::Env::new();
    env = env.filter_or("GOLDFISH_LOG_LEVEL", "info");
    env_logger::Builder::from_env(env).try_init().unwrap();

    let cli = Cli::parse();
    let app_settings = Settings::with_overrides(cli.host, cli.port)?;

    run(app_settings).await
}
