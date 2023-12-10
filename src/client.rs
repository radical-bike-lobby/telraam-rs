//! Client library, based on reqwest, this sets up connection with required parameters for the Telraam API endpoints

use std::error::Error;

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE},
};

use crate::endpoint::Endpoint;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

const TELRAAM_NET: &str = "https://telraam-api.net";

/// An HTTPS Client for working with the Telraam API
pub struct TelraamClient(Client);

impl TelraamClient {
    /// Constructs a new Client
    ///
    /// # Arguments
    ///
    /// * `new` - The API token from [Telraam](https://telraam.net/en/admin/mijn-eigen-telraam/tokens) for this connection.
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

    /// Send a request to the given endpoint, the response is endpoint specific
    ///
    /// # Argument
    ///
    /// * `endpoint` - The endpoint to use for the connection
    ///
    /// # Returns
    ///
    /// The result is endpoint specific, but will always be serializable, see `serde_json::to_string_pretty`
    pub fn send<E: Endpoint>(&self, endpoint: &E) -> Result<E::Response, Box<dyn Error>> {
        let mut url = format!(
            "{base}/{version}/{endpoint}",
            base = TELRAAM_NET,
            version = crate::VER,
            endpoint = E::PATH
        );

        // add the path params, for things like instance IDs
        if let Some(path_params) = endpoint.path_params() {
            url.push('/');
            url.push_str(path_params)
        };

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
