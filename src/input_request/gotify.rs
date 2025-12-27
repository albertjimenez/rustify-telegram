use actix_web::{post, web, HttpRequest, HttpResponse};
use log::{info, warn};

use crate::{
    config::app_config::AppConfig,
    input_request::gotify_message::GotifyMessage,
    services::telegram,
};
use crate::auth::extract_token::extract_token;

#[post("/message")]
pub async fn input_request_gotify(
    req: HttpRequest,
    payload: web::Json<GotifyMessage>,
    client: web::Data<reqwest::Client>,
    config: web::Data<AppConfig>,
) -> HttpResponse {
    info!(
        "Gotify message received: title='{}'",
        &payload.title,
    );

    let token = extract_token(&req);

    if token != Some(config.gotify_token.as_str()) {
        warn!("Unauthorized Gotify request");
        return HttpResponse::Unauthorized().finish();
    }

    match telegram::send_message(
        &client,
        &config.telegram_token,
        &config.telegram_chat_id,
        &payload.title,
        &payload.message,
    )
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
