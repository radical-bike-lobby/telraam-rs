use std::error::Error;

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE},
};

use crate::endpoint::Endpoint;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

const TELRAAM_NET: &str = "https://telraam-api.net";

pub struct TelraamClient(Client);

impl TelraamClient {
    pub fn new(api_token: &str) -> Result<Self, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        let mut api_token = HeaderValue::from_str(api_token)?;
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        api_token.set_sensitive(true);
        headers.insert("X-Api-Key", api_token);

        let client = reqwest::blocking::ClientBuilder::new()
            .user_agent(APP_USER_AGENT)
            .default_headers(headers)
            .build()?;

        Ok(Self(client))
    }

    pub fn send<E: Endpoint>(&self, endpoint: &E) -> Result<E::Response, Box<dyn Error>> {
        let url = format!(
            "{base}/{version}/{endpoint}",
            base = TELRAAM_NET,
            version = crate::VER,
            endpoint = E::PATH
        );

        let request = self.0.request(E::METHOD, url).query(&endpoint.params());

        let request = if let Some(payload) = endpoint.payload() {
            let body = serde_json::to_string(&payload)?;
            request.body(body)
        } else {
            request
        };

        Ok(request.send()?.json()?)
    }
}
