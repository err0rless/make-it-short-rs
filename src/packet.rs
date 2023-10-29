use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ShortenURLReq {
    pub url: String,
}

#[derive(Serialize)]
pub struct ShortenURLRes {
    pub short_url: String,
}
