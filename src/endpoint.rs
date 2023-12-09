use std::collections::HashMap;

use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Map, Value};

use crate::response::{Response, WelcomeResponse};

/// Endpoint is a trait that defines the shape of all the API endpoints in Telraam
///
/// An instance of the Endpoint will contain the parameters to be used for any subsequent queries.
pub trait Endpoint {
    const PATH: &'static str;
    const METHOD: Method;

    type Response: Response + DeserializeOwned;
    type Request: Serialize;

    /// Payload should only be associated for POST, PUT, or PATCH requests
    fn payload(&self) -> Option<&Self::Request> {
        None
    }

    /// Parameters to add to the request
    fn params(&self) -> HashMap<String, Option<String>> {
        HashMap::new()
    }
}

pub struct Welcome;

impl Endpoint for Welcome {
    const PATH: &'static str = "";
    const METHOD: Method = Method::GET;

    type Response = WelcomeResponse;
    type Request = ();
}
