mod config;
mod error;
mod handlers;
mod routes;
mod services;
mod states;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!(
        "starting {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    info!("building configuartion");
    let app_config = config::AppConfig::new();

    info!("initializing states");
    let ip_counter = states::IpCounter::new();

    info!("binding server to {}:{}", app_config.host, app_config.port);
    HttpServer::new(move || {
        App::new()
            .app_data(ip_counter.clone())
            .configure(routes::configuration)
    })
    .bind((app_config.host, app_config.port))?
    .run()
    .await
}
