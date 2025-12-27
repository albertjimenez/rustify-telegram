use log::{error, info};
use reqwest::Client;

pub async fn send_message(
    client: &Client,
    bot_token: &str,
    chat_id: &str,
    title: &str,
    message: &str,
) -> Result<(), ()> {
    let text = format!("🔔 *{}*\n{}", title, message);

    let res = client
        .post(format!(
            "https://api.telegram.org/bot{}/sendMessage",
            bot_token
        ))
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "text": text,
            "parse_mode": "Markdown"
        }))
        .send()
        .await;

    match res {
        Ok(_) => {
            info!("Telegram notification sent");
            Ok(())
        }
        Err(e) => {
            error!("Telegram send failed: {:?}", e);
            Err(())
        }
    }
}
