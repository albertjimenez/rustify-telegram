use actix_web::{App, HttpServer, web};
use log::info;
use reqwest::Client;
use rustify_telegram::input_request::input_request::input_request_gotify;
use rustify_telegram::data::env_validator::EnvValidator;
use web::Data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let port_str = std::env::var("PORT").unwrap_or_else(|_| "8396".to_string());
    let port = port_str.parse().unwrap_or(8396);
    EnvValidator::default().validate();
    let client = Client::new();
    info!("starting server");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .service(input_request_gotify)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
