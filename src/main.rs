use actix_web::HttpServer;
use log::info;
use env_logger::Env;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct CliArgs {
    #[arg(long, default_value = "0.0.0.0")]
    host: String,
    #[arg(long, default_value = "2222")]
    port: u16,
    #[arg(long, default_value = "./config.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting server at http://{}:{}", args.host, args.port);

    HttpServer::new(|| {
        actix_web::App::new()
            .route("/", actix_web::web::get().to(|| async { "Hello, world!" }))
    })
    .bind((args.host, args.port))?
    .run()
    .await
}