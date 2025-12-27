use actix_web::{App, HttpServer, web};
use env_logger::Env;
use log::info;
use reqwest::Client;
use rustify_telegram::config::app_config::AppConfig;
use rustify_telegram::data::env_names::EnvNames;
use rustify_telegram::data::env_validator::EnvValidator;
use rustify_telegram::input_request::gotify::input_request_gotify;
use web::Data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting rustify-telegram");
    EnvValidator::default().validate();

    let config = AppConfig::from_env();
    let port = EnvNames::Port.get_u16();
    let client = Client::new();
    info!("Listening on 0.0.0.0:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .app_data(Data::new(config.clone()))
            .service(input_request_gotify)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
