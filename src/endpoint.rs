use std::{collections::HashMap, time::SystemTime};

use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize, Serializer};

use crate::response::{Response, TrafficResponse, WelcomeResponse};

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

pub struct Traffic;

impl Endpoint for Traffic {
    const PATH: &'static str = "reports/traffic";
    const METHOD: Method = Method::POST;

    type Response = TrafficResponse;
    type Request = TrafficRequest;
}

#[derive(Serialize)]
pub struct TrafficRequest {
    level: TrafficLevel,
    format: String,
    id: String,
    #[serde(serialize_with = "format_rfc3339_millis")]
    time_start: SystemTime,
    #[serde(serialize_with = "format_rfc3339_millis")]
    time_end: SystemTime,
}

fn format_rfc3339_millis<S: Serializer>(
    time: &SystemTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let time = humantime::format_rfc3339_millis(*time);
    serializer.serialize_str(&time.to_string())
}

#[derive(Default, Serialize)]
pub enum TrafficLevel {
    #[default]
    #[serde(rename = "segments")]
    Segments,
    #[serde(rename = "instance")]
    Instance,
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    #[test]
    fn test_serialize_traffic() {
        let request = TrafficRequest {
            level: TrafficLevel::Segments,
            format: String::from("per-hour"),
            id: "348917".to_string(),
            time_start: humantime::parse_rfc3339_weak("2020-10-30 07:00:00Z").unwrap(),
            time_end: humantime::parse_rfc3339_weak("2020-10-30 09:00:00Z").unwrap(),
        };
        let json = serde_json::to_string_pretty(&request).expect("failed to serialize");
        let parsed = serde_json::from_str::<Value>(&json).expect("failed to parse");

        assert_eq!("segments", parsed["level"]);
        assert_eq!("per-hour", parsed["format"]);
        assert_eq!("348917", parsed["id"]);
        // hopefully this formatting isn't a problem, notice the spaces in the original
        assert_eq!("2020-10-30T07:00:00.000Z", parsed["time_start"]);
        assert_eq!("2020-10-30T09:00:00.000Z", parsed["time_end"]);
    }
}
