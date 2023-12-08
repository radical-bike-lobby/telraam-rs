use serde::Deserialize;

#[derive(Deserialize)]
pub struct WelcomeResponse {
    pub msg: String,
}
