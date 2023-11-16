use actix_web::HttpServer;
use log::{debug, info};
use env_logger::Env;
use clap::Parser;
use tail::config::config::load;

#[derive(Debug, Parser)]
pub struct CliArgs {
    #[arg(long, default_value = "./config.toml")]
    config: String,
    #[arg(long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();

    if args.debug {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = load(&args.config);
    debug!("Loaded config: {:?}", config);

    info!("Starting server at http://{}:{}", config.server.ip, config.server.port);

    HttpServer::new(|| {
        actix_web::App::new()
            .configure(|cfg| tail::app_config(cfg))
    })
    .bind((config.server.ip, config.server.port))?
    .run()
    .await
}