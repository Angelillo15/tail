use actix_web::HttpServer;
use log::{debug, error, info};
use env_logger::Env;
use clap::Parser;
use sea_orm::{Database, DatabaseConnection, DbErr};
use migration::{Migrator, MigratorTrait};
use tail::config::config::{Data, load};

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

    let conn = connect_to_database(&config).await;

    HttpServer::new(|| {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .configure(|cfg| tail::app_config(cfg))
    })
        .bind((config.server.ip, config.server.port))?
        .run()
        .await
}

async fn connect_to_database(config: &Data) -> DatabaseConnection {
    let conn = match Database::connect(&config.database_url).await {
        Ok(c) => c,
        Err(error) => {
            error!("Unable to connect to database");
            error!("Error: {}", error);
            std::process::exit(1);
        }
    };

    match Migrator::up(&conn, None).await {
        Ok(_) => {
            debug!("Successfully migrated database");
        }
        Err(error) => {
            error!("Unable to migrate database");
            error!("Error: {}", error);
            std::process::exit(1);
        }
    };

    conn
}