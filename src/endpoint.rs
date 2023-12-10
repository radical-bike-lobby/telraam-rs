use std::{collections::HashMap, time::SystemTime};

#[cfg(feature = "clap")]
use clap::{Args, Parser, ValueEnum};
use reqwest::Method;
use serde::{Serialize, Serializer};

use crate::response::{
    CamerasResponse, Response, SegmentResponse, TrafficResponse, WelcomeResponse,
};

/// Endpoint is a trait that defines the shape of all the API endpoints in Telraam
///
/// An instance of the Endpoint will contain the parameters to be used for any subsequent queries.
pub trait Endpoint {
    const PATH: &'static str;
    const METHOD: Method;

    type Response: Response;
    type Request: Serialize;

    /// Payload should only be associated for POST, PUT, or PATCH requests
    fn payload(&self) -> Option<&Self::Request> {
        None
    }

    /// Parameters to add to the request
    fn params(&self) -> HashMap<String, Option<String>> {
        HashMap::new()
    }

    /// Path params additional parameters to add to the path
    fn path_params(&self) -> Option<&str> {
        None
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "clap", derive(Parser))]
pub struct Welcome;

impl Endpoint for Welcome {
    const PATH: &'static str = "";
    const METHOD: Method = Method::GET;

    type Response = WelcomeResponse;
    type Request = ();
}

#[derive(Debug)]
#[cfg_attr(feature = "clap", derive(Parser))]
pub struct Traffic {
    #[cfg_attr(feature = "clap", command(flatten))]
    request: TrafficRequest,
}

impl Endpoint for Traffic {
    const PATH: &'static str = "reports/traffic";
    const METHOD: Method = Method::POST;

    type Response = TrafficResponse;
    type Request = TrafficRequest;

    fn payload(&self) -> Option<&Self::Request> {
        Some(&self.request)
    }
}

#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "clap", derive(Args))]
pub struct TrafficRequest {
    level: TrafficLevel,
    format: String,
    id: String,
    #[serde(serialize_with = "format_rfc3339_millis")]
    #[cfg_attr(feature = "clap", arg(value_parser = humantime::parse_rfc3339_weak))]
    time_start: SystemTime,
    #[serde(serialize_with = "format_rfc3339_millis")]
    #[cfg_attr(feature = "clap", arg(value_parser = humantime::parse_rfc3339_weak))]
    time_end: SystemTime,
}

fn format_rfc3339_millis<S: Serializer>(
    time: &SystemTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let time = humantime::format_rfc3339_millis(*time);
    serializer.serialize_str(&time.to_string())
}

#[derive(Clone, Debug, Default, Serialize)]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum TrafficLevel {
    #[default]
    #[serde(rename = "segments")]
    Segments,
    #[serde(rename = "instance")]
    Instance,
}

#[derive(Debug)]
#[cfg_attr(feature = "clap", derive(Parser))]
pub struct LiveTrafficSnapshot;

impl Endpoint for LiveTrafficSnapshot {
    const PATH: &'static str = "reports/traffic_snapshot_live";
    const METHOD: Method = Method::GET;

    type Response = TrafficResponse;
    type Request = ();
}

#[derive(Debug)]
#[cfg_attr(feature = "clap", derive(Parser))]
pub struct AllAvailableCameras;

impl Endpoint for AllAvailableCameras {
    const PATH: &'static str = "cameras";
    const METHOD: Method = Method::GET;

    type Response = CamerasResponse;
    type Request = ();
}

pub struct CamerasBySegmentId {
    segment_id: String,
}

impl Endpoint for CamerasBySegmentId {
    const PATH: &'static str = "cameras/segment";
    const METHOD: Method = Method::GET;

    type Response = CamerasResponse;
    type Request = ();

    fn path_params(&self) -> Option<&str> {
        Some(&self.segment_id)
    }
}

pub struct CameraByMacId {
    mac_id: String,
}

impl Endpoint for CameraByMacId {
    const PATH: &'static str = "cameras";
    const METHOD: Method = Method::GET;

    type Response = CamerasResponse;
    type Request = ();

    fn path_params(&self) -> Option<&str> {
        Some(&self.mac_id)
    }
}

pub struct AllSegments;

impl Endpoint for AllSegments {
    const PATH: &'static str = "segments/all";
    const METHOD: Method = Method::GET;

    type Response = SegmentResponse;
    type Request = ();
}

pub struct SegmentById {
    segment_id: String,
}

impl Endpoint for SegmentById {
    const PATH: &'static str = "segments/id";
    const METHOD: Method = Method::GET;

    type Response = SegmentResponse;
    type Request = ();

    fn path_params(&self) -> Option<&str> {
        Some(&self.segment_id)
    }
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
