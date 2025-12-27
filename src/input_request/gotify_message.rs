use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GotifyMessage {
    pub(crate) title: String,
    pub(crate) message: String,
    priority: Option<i32>,
}